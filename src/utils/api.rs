use crate::utils::errors::abort;
use crate::utils::http::HTTP_CLIENT;
use anyhow::{bail, Context, Result};
use const_format::concatcp;
use elasticsearch_dsl::{
    Aggregation, FieldSort, Operator, Query, Search, SearchResponse, Sort, TextQueryType,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiErrorResponseError,
    // pub status: u16,
}
#[derive(Deserialize)]
pub struct ApiErrorResponseError {
    pub reason: String,
}

#[derive(Deserialize)]
pub struct ApiPackage {
    #[serde(rename = "package_pversion")]
    pub pversion: String,
    #[serde(rename = "package_attr_name")]
    pub attr_name: String,
    #[serde(rename = "package_description")]
    pub description: Option<String>,
    #[serde(rename = "package_longDescription")]
    pub long_description: Option<String>,
    #[serde(rename = "package_homepage")]
    pub homepage: Vec<String>,
    #[serde(rename = "package_position")]
    pub position: Option<String>,
}

const API_BASE_URL: &str = "https://search.nixos.org/backend/";

static API_URL: LazyLock<String> = LazyLock::new(|| {
    let api_alias = match get_api_alias() {
        Ok(url) => url,
        Err(err) => {
            abort(err);
        }
    };
    format!("{}/{}/_search", API_BASE_URL, api_alias)
});

fn get_api_alias() -> Result<String> {
    let response = HTTP_CLIENT
        .get(concatcp!(API_BASE_URL, "_aliases"))
        .basic_auth("aWVSALXpZv", Some("X8gPHnzL52wFEekuxsfQ9cSh"))
        .send()
        .context("Failed to fetch aliases from nixos.org")?;

    if !response.status().is_success() {
        let message = response.json::<ApiErrorResponse>().map_or(
            "An error occuried while fetching aliases from nixos.org".into(),
            |response| response.error.reason,
        );
        bail!(message);
    }

    #[derive(Debug, Deserialize)]
    struct Empty {}
    #[derive(Debug, Deserialize)]
    struct Aliases {
        aliases: HashMap<String, Empty>,
    }
    type AllAliases = HashMap<String, Aliases>;

    let all_aliases = response
        .json::<AllAliases>()
        .context("Failed to parse json")?;

    let alias = all_aliases
        .into_values()
        .flat_map(|aliases| aliases.aliases.into_keys())
        .find(|alias| alias.starts_with("latest") && alias.ends_with("nixos-unstable"))
        .context("Failed to find latest unstable alias")?;

    Ok(alias)
}

fn fetch_api(query: Search) -> Result<Vec<ApiPackage>> {
    let response = HTTP_CLIENT
        .post(&*API_URL)
        .json(&query)
        .header("User-Agent", "nw/0.1.0")
        .basic_auth("aWVSALXpZv", Some("X8gPHnzL52wFEekuxsfQ9cSh"))
        .send()
        .context("Failed to fetch result from nixos.org")?;

    if !response.status().is_success() {
        let message = response.json::<ApiErrorResponse>().map_or(
            "An error occuried while fetching result from nixos.org".into(),
            |response| response.error.reason,
        );
        bail!(message);
    }

    let json = response
        .json::<SearchResponse>()
        .context("Failed to retrieve json")?;
    let packages = json
        .documents::<ApiPackage>()
        .context("Failed to parse json")?;

    Ok(packages)
}

pub fn get_by_query(query: String) -> Result<Vec<ApiPackage>> {
    let query_str = format!("*{}*", query);
    let query_search = Search::new()
        .from(0)
        .size(30)
        .query(
            Query::bool().filter(Query::term("type", "package")).must(
                Query::dis_max()
                    .tie_breaker(0.7)
                    .query(
                        Query::multi_match(
                            [
                                "package_attr_name^9",
                                "package_attr_name.*^5.3999999999999995",
                                "package_programs^9",
                                "package_programs.*^5.3999999999999995",
                                "package_pname^6",
                                "package_pname.*^3.5999999999999996",
                                "package_description^1.3",
                                "package_description.*^0.78",
                                "package_longDescription^1",
                                "package_longDescription.*^0.6",
                                "flake_name^0.5",
                                "flake_name.*^0.3",
                            ],
                            query,
                        )
                        .r#type(TextQueryType::CrossFields)
                        .analyzer("whitespace")
                        .auto_generate_synonyms_phrase_query(false)
                        .operator(Operator::And),
                    )
                    .query(Query::wildcard("package_attr_name", query_str).case_insensitive(true)),
            ),
        )
        .sort([
            Sort::FieldSort(FieldSort::descending("_score")),
            Sort::FieldSort(FieldSort::descending("package_attr_name")),
            Sort::FieldSort(FieldSort::descending("package_pversion")),
        ])
        .aggregate(
            "package_attr_set",
            Aggregation::Terms(Aggregation::terms("package_attr_set")),
        )
        .aggregate(
            "package_license_set",
            Aggregation::Terms(Aggregation::terms("package_license_set")),
        )
        .aggregate(
            "package_maintainers_set",
            Aggregation::Terms(Aggregation::terms("package_maintainers_set")),
        )
        .aggregate(
            "package_platforms",
            Aggregation::Terms(Aggregation::terms("package_platforms")),
        );

    let packages = fetch_api(query_search)?;

    Ok(packages)
}

pub fn get_by_attr_name(name: String) -> Result<Option<ApiPackage>> {
    let query = Search::new().query(Query::r#match("package_attr_name", name));
    let packages = fetch_api(query)?;
    let package = packages.into_iter().next();
    Ok(package)
}

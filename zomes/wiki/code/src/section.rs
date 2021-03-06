extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::holochain_core_types::{
    dna::entry_types::Sharing,
    entry::Entry,
    // agent::AgentId, dna::entry_types::Sharing, entry::Entry, link::LinkMatch,
};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::Entry::App;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    //,
    // AGENT_ADDRESS, AGENT_ID_STR,
};

use crate::page;
use crate::page::Page;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum Content {
    Text(String),
    Binarys(Vec<u8>),
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section {
    page_address: Address,
    r#type: String,
    content: String,
    rendered_content: String,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section2 {
    r#type: String,
    content: String,
    rendered_content: String,
}

impl Section {
    pub fn from(section: Section2, page_address: Address) -> Section {
        Section {
            page_address,
            r#type: section.r#type,
            content: section.content,
            rendered_content: section.rendered_content,
        }
    }
    pub fn entry(self) -> Entry {
        App("pageElement".into(), self.into())
    }
}
pub fn page_element_def() -> ValidatingEntryType {
    entry!(
        name: "pageElement",
        description: "this is an entry representing some profile info for an agent",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Section>| {
            Ok(())
        }
    )
}
pub fn update_element(address: Address, element: Section2) -> ZomeApiResult<Address> {
    let old_element = hdk::utils::get_as_type::<Section>(address.clone())?;

    hdk::api::update_entry(
        Section::from(element, old_element.page_address).entry(),
        &address,
    )
}
pub fn delete_element(address: Address) -> ZomeApiResult<String> {
    let page_address = hdk::utils::get_as_type::<Section>(address.clone())?.page_address;
    //hdk::api::remove_entry(&address)?;

    let page = hdk::utils::get_as_type::<Page>(page_address.clone())?;
    let sections = page
        .clone()
        .sections
        .into_iter()
        .filter_map(|d_address| {
            if d_address != address {
                Some(d_address)
            } else {
                None
            }
        })
        .collect();
    hdk::api::update_entry(
        Page::from(page.title.clone(), sections).entry(),
        &page_address,
    )?;

    Ok(page.title)
}
pub fn add_section(title: String, element: Section2) -> ZomeApiResult<Address> {
    let page_address = page::create_page_if_non_existent(title.clone())?;
    hdk::api::commit_entry(&Section::from(element, page_address).entry())
}

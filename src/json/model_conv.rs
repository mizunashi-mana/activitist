use std::{collections::HashMap, error::Error};

use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::model;

pub trait ModelConv
where
    Self: Sized,
{
    type JsonSerdeValue: Serialize + DeserializeOwned;

    fn from_model(&self) -> Result<Self::JsonSerdeValue, Box<dyn Error>>;
    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>>;
}

impl ModelConv for model::Object {
    type JsonSerdeValue = Object;

    fn from_model(&self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        let (inbox, outbox, followers, following, preferred_username, endpoints) =
            match &self.actor_items {
                Some(item) => (
                    Some(&item.inbox),
                    Some(&item.outbox),
                    Some(&item.followers),
                    Some(&item.following),
                    item.preferred_username.as_ref(),
                    if item.endpoints.is_empty() {
                        None
                    } else {
                        Some(&item.endpoints)
                    },
                ),
                None => (None, None, None, None, None, None),
            };

        Ok(Object {
            schema_context: from_model_opt(self.schema_context.as_ref())?,
            id: self.id.clone(),
            typ: to_lax_array(&self.typ)?,
            attachment: to_lax_array(&self.object_items.attachment)?,
            attributed_to: to_lax_array(&self.object_items.attributed_to)?,
            audience: to_lax_array(&self.object_items.audience)?,
            bcc: to_lax_array(&self.object_items.bcc)?,
            bto: to_lax_array(&self.object_items.bto)?,
            cc: to_lax_array(&self.object_items.cc)?,
            context: to_lax_array(&self.object_items.context)?,
            generator: to_lax_array(&self.object_items.generator)?,
            icon: to_lax_array(&self.object_items.icon)?,
            image: to_lax_array(&self.object_items.image)?,
            in_reply_to: to_lax_array(&self.object_items.in_reply_to)?,
            location: to_lax_array(&self.object_items.location)?,
            preview: to_lax_array(&self.object_items.preview)?,
            replies: boxed_from_model_opt(self.object_items.replies.as_ref())?,
            tag: to_lax_array(&self.object_items.tag)?,
            to: to_lax_array(&self.object_items.to)?,
            url: match &self.object_items.url {
                None => None,
                Some(item) => {
                    if {
                        item.height.is_none()
                            && item.hreflang.is_none()
                            && item.id.is_none()
                            && item.media_type.is_empty()
                            && item.rel.is_empty()
                            && item.typ.is_empty()
                            && item.width.is_none()
                    } {
                        Some(Value::String(item.href.clone()))
                    } else {
                        Some(serde_json::to_value(item.from_model()?)?)
                    }
                }
            },
            content: to_lax_array(&self.object_items.content)?,
            content_map: if self.object_items.content_map.is_empty() {
                None
            } else {
                Some(self.object_items.content_map.clone())
            },
            name: to_lax_array(&self.object_items.name)?,
            name_map: if self.object_items.name_map.is_empty() {
                None
            } else {
                Some(self.object_items.name_map.clone())
            },
            duration: self.object_items.duration.clone(),
            media_type: to_lax_array(&self.object_items.media_type)?,
            end_time: from_model_opt(self.object_items.end_time.as_ref())?,
            published: from_model_opt(self.object_items.published.as_ref())?,
            summary: to_lax_array(&self.object_items.summary)?,
            summary_map: if self.object_items.summary_map.is_empty() {
                None
            } else {
                Some(self.object_items.summary_map.clone())
            },
            updated: from_model_opt(self.object_items.updated.as_ref())?,
            describes: boxed_from_model_opt(self.object_items.describes.as_ref())?,
            inbox: inbox.cloned(),
            outbox: outbox.cloned(),
            followers: followers.cloned(),
            following: following.cloned(),
            preferred_username: preferred_username.cloned(),
            endpoints: endpoints.cloned(),
            actor: to_lax_array(&self.activity_items.actor)?,
            instrument: to_lax_array(&self.activity_items.instrument)?,
            origin: to_lax_array(&self.activity_items.origin)?,
            object: to_lax_array(&self.activity_items.object)?,
            result: to_lax_array(&self.activity_items.result)?,
            target: to_lax_array(&self.activity_items.target)?,
            total_items: self.collection_items.total_items,
            current: boxed_from_model_opt(self.collection_items.current.as_ref())?,
            first: boxed_from_model_opt(self.collection_items.first.as_ref())?,
            last: boxed_from_model_opt(self.collection_items.last.as_ref())?,
            items: to_lax_array(&self.collection_items.items)?,
            ordered_items: to_lax_array(&self.ordered_collection_items.ordered_items)?,
            next: boxed_from_model_opt(self.collection_page_items.next.as_ref())?,
            prev: boxed_from_model_opt(self.collection_page_items.prev.as_ref())?,
            part_of: boxed_from_model_opt(self.collection_page_items.part_of.as_ref())?,
            start_index: self.ordered_collection_page_items.start_index,
            subject: boxed_from_model_opt(self.relationship_items.subject.as_ref())?,
            relationship: to_lax_array(&self.relationship_items.relationship)?,
            former_type: to_lax_array(&self.tombstone_items.former_type)?,
            deleted: from_model_opt(self.tombstone_items.deleted.as_ref())?,
            one_of: to_lax_array(&self.question_items.one_of)?,
            any_of: to_lax_array(&self.question_items.any_of)?,
            closed: self.question_items.closed.clone(),
            accuracy: self.place_items.accuracy,
            altitude: self.place_items.altitude,
            latitute: self.place_items.latitute,
            longitute: self.place_items.longitute,
            radius: self.place_items.radius,
            units: self.place_items.units.clone(),
            manually_approves_followers: self
                .activity_streams_ext_items
                .manually_approves_followers,
            also_known_as: to_lax_array(&self.activity_streams_ext_items.also_known_as)?,
            moved_to: self.activity_streams_ext_items.moved_to.clone(),
            sensitive: self.activity_streams_ext_items.sensitive,
            featured: self.mastodon_ext_items.featured.clone(),
            featured_tags: self.mastodon_ext_items.featured_tags.clone(),
            discoverable: self.mastodon_ext_items.discoverable,
            suspended: self.mastodon_ext_items.suspended,
            devices: self.mastodon_ext_items.devices.clone(),
            public_key: from_model_opt(self.security_items.public_key.as_ref())?,
            value: self.property_items.value.clone(),
        })
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            schema_context: to_model_opt(origin.schema_context)?,
            id: origin.id,
            typ: from_lax_array(origin.typ)?,
            object_items: model::ObjectItems {
                attachment: from_lax_array(origin.attachment)?,
                attributed_to: from_lax_array(origin.attributed_to)?,
                audience: from_lax_array(origin.audience)?,
                bcc: from_lax_array(origin.bcc)?,
                bto: from_lax_array(origin.bto)?,
                cc: from_lax_array(origin.cc)?,
                context: from_lax_array(origin.context)?,
                generator: from_lax_array(origin.generator)?,
                icon: from_lax_array(origin.icon)?,
                image: from_lax_array(origin.image)?,
                in_reply_to: from_lax_array(origin.in_reply_to)?,
                location: from_lax_array(origin.location)?,
                preview: from_lax_array(origin.preview)?,
                replies: boxed_to_model_opt(origin.replies)?,
                tag: from_lax_array(origin.tag)?,
                to: from_lax_array(origin.to)?,
                url: match origin.url {
                    None => None,
                    Some(Value::String(item)) => Some(model::Link::from(item)),
                    Some(item) => {
                        let item: Link = serde_json::from_value(item)?;
                        Some(model::Link::to_model(item)?)
                    }
                },
                content: from_lax_array(origin.content)?,
                content_map: match origin.content_map {
                    None => HashMap::new(),
                    Some(item) => item,
                },
                name: from_lax_array(origin.name)?,
                name_map: match origin.name_map {
                    None => HashMap::new(),
                    Some(item) => item,
                },
                duration: origin.duration,
                media_type: from_lax_array(origin.media_type)?,
                end_time: to_model_opt(origin.end_time)?,
                published: to_model_opt(origin.published)?,
                summary: from_lax_array(origin.summary)?,
                summary_map: match origin.summary_map {
                    None => HashMap::new(),
                    Some(item) => item,
                },
                updated: to_model_opt(origin.updated)?,
                describes: boxed_to_model_opt(origin.describes)?,
            },
            actor_items: match (
                origin.inbox,
                origin.outbox,
                origin.followers,
                origin.following,
            ) {
                (Some(inbox), Some(outbox), Some(followers), Some(following)) => {
                    Some(model::ActorItems {
                        inbox,
                        outbox,
                        following,
                        followers,
                        preferred_username: origin.preferred_username,
                        endpoints: match origin.endpoints {
                            None => HashMap::new(),
                            Some(item) => item,
                        },
                    })
                }
                _ => None,
            },
            activity_items: model::ActivityItems {
                actor: from_lax_array(origin.actor)?,
                instrument: from_lax_array(origin.instrument)?,
                origin: from_lax_array(origin.origin)?,
                object: from_lax_array(origin.object)?,
                result: from_lax_array(origin.result)?,
                target: from_lax_array(origin.target)?,
            },
            collection_items: model::CollectionItems {
                total_items: origin.total_items,
                current: boxed_to_model_opt(origin.current)?,
                first: boxed_to_model_opt(origin.first)?,
                last: boxed_to_model_opt(origin.last)?,
                items: from_lax_array(origin.items)?,
            },
            ordered_collection_items: model::OrderedCollectionItems {
                ordered_items: from_lax_array(origin.ordered_items)?,
            },
            collection_page_items: model::CollectionPageItems {
                next: boxed_to_model_opt(origin.next)?,
                prev: boxed_to_model_opt(origin.prev)?,
                part_of: boxed_to_model_opt(origin.part_of)?,
            },
            ordered_collection_page_items: model::OrderedCollectionPageItems {
                start_index: origin.start_index,
            },
            relationship_items: model::RelationshipItems {
                subject: boxed_to_model_opt(origin.subject)?,
                relationship: from_lax_array(origin.relationship)?,
            },
            tombstone_items: model::TombstoneItems {
                former_type: from_lax_array(origin.former_type)?,
                deleted: to_model_opt(origin.deleted)?,
            },
            question_items: model::QuestionItems {
                one_of: from_lax_array(origin.one_of)?,
                any_of: from_lax_array(origin.any_of)?,
                closed: origin.closed,
            },
            place_items: model::PlaceItems {
                accuracy: origin.accuracy,
                altitude: origin.altitude,
                latitute: origin.latitute,
                longitute: origin.longitute,
                radius: origin.radius,
                units: origin.units,
            },
            activity_streams_ext_items: model::ActivityStreamExtItems {
                manually_approves_followers: origin.manually_approves_followers,
                also_known_as: from_lax_array(origin.also_known_as)?,
                moved_to: origin.moved_to,
                sensitive: origin.sensitive,
            },
            mastodon_ext_items: model::MastodonExtItems {
                featured: origin.featured,
                featured_tags: origin.featured_tags,
                discoverable: origin.discoverable,
                suspended: origin.suspended,
                devices: origin.devices,
            },
            security_items: model::SecurityItems {
                public_key: to_model_opt(origin.public_key)?,
            },
            property_items: model::PropertyItems {
                value: origin.value,
            },
        })
    }
}

impl ModelConv for model::Link {
    type JsonSerdeValue = Link;

    fn from_model(&self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        Ok(Link {
            schema_context: from_model_opt(self.schema_context.as_ref())?,
            id: self.id.clone(),
            typ: to_lax_array(&self.typ)?,
            href: self.href.clone(),
            height: self.height,
            hreflang: self.hreflang.clone(),
            media_type: to_lax_array(&self.media_type)?,
            rel: to_lax_array(&self.rel)?,
            width: self.width,
        })
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        Ok(model::Link {
            schema_context: to_model_opt(origin.schema_context)?,
            id: origin.id,
            typ: from_lax_array(origin.typ)?,
            href: origin.href,
            height: origin.height,
            hreflang: origin.hreflang,
            media_type: from_lax_array(origin.media_type)?,
            rel: from_lax_array(origin.rel)?,
            width: origin.width,
        })
    }
}

impl ModelConv for model::ObjectOrLink {
    type JsonSerdeValue = ObjectOrLink;

    fn from_model(&self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        match self {
            Self::Object(origin) => Ok(ObjectOrLink::Object(origin.from_model()?)),
            Self::Link(origin) => {
                if {
                    origin.height.is_none()
                        && origin.hreflang.is_none()
                        && origin.id.is_none()
                        && origin.media_type.is_empty()
                        && origin.rel.is_empty()
                        && origin.typ.is_empty()
                        && origin.width.is_none()
                } {
                    Ok(ObjectOrLink::Uri(origin.href.clone()))
                } else {
                    Ok(ObjectOrLink::Link(origin.from_model()?))
                }
            }
        }
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        match origin {
            ObjectOrLink::Link(origin) => Ok(Self::Link(model::Link::to_model(origin)?)),
            ObjectOrLink::Uri(origin) => Ok(Self::Link(model::Link::from(origin))),
            ObjectOrLink::Object(origin) => Ok(Self::Object(model::Object::to_model(origin)?)),
        }
    }
}

impl ModelConv for model::Context {
    type JsonSerdeValue = Context;

    fn from_model(&self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        match self {
            Self::Single(origin) => Ok(Context::Single(origin.from_model()?)),
            Self::Mix(origin) => {
                let mut dest = Vec::with_capacity(origin.len());
                for item in origin {
                    dest.push(item.from_model()?);
                }
                Ok(Context::Mix(dest))
            }
            Self::TermDefs(origin) => {
                let mut dest = HashMap::with_capacity(origin.len());
                for (key, item) in origin {
                    dest.insert(key.clone(), item.from_model()?);
                }
                Ok(Context::TermDefs(dest))
            }
        }
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        match origin {
            Context::Single(origin) => Ok(model::Context::Single(ModelConv::to_model(origin)?)),
            Context::Mix(origin) => {
                let mut dest = Vec::with_capacity(origin.len());
                for item in origin {
                    dest.push(ModelConv::to_model(item)?)
                }
                Ok(model::Context::Mix(dest))
            }
            Context::TermDefs(origin) => {
                let mut dest = HashMap::with_capacity(origin.len());
                for (key, item) in origin {
                    dest.insert(key, ModelConv::to_model(item)?);
                }
                Ok(model::Context::TermDefs(dest))
            }
        }
    }
}

impl ModelConv for model::Iri {
    type JsonSerdeValue = Iri;

    fn from_model(&self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        match self {
            Self::Direct(origin) => Ok(Iri::Direct(origin.clone())),
            Self::TypeCoercion { id, typ } => Ok(Iri::TypeCoercion(TypeCoercion {
                id: id.clone(),
                typ: typ.clone(),
            })),
        }
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        match origin {
            Iri::Direct(origin) => Ok(model::Iri::Direct(origin)),
            Iri::TypeCoercion(origin) => Ok(model::Iri::TypeCoercion {
                id: origin.id,
                typ: origin.typ,
            }),
        }
    }
}

impl ModelConv for model::Key {
    type JsonSerdeValue = Key;

    fn from_model(&self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        Ok(Key {
            id: self.id.clone(),
            owner: self.owner.clone(),
            public_key_pem: self.public_key_pem.clone(),
        })
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            id: origin.id,
            owner: origin.owner,
            public_key_pem: origin.public_key_pem,
        })
    }
}

impl ModelConv for String {
    type JsonSerdeValue = String;

    fn from_model(&self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        Ok(self.clone())
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        Ok(origin)
    }
}

impl ModelConv for DateTime<Utc> {
    type JsonSerdeValue = String;

    fn from_model(&self) -> Result<Self::JsonSerdeValue, Box<dyn Error>> {
        Ok(self.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
    }

    fn to_model(origin: Self::JsonSerdeValue) -> Result<Self, Box<dyn Error>> {
        Ok(DateTime::parse_from_rfc3339(&origin)?.with_timezone(&Utc))
    }
}

pub fn to_lax_array<T: ModelConv>(origin: &[T]) -> Result<Option<Value>, Box<dyn Error>> {
    match origin.len() {
        0 | 1 => {
            for item in origin {
                return Ok(Some(serde_json::to_value(item.from_model()?)?));
            }
            Ok(None)
        }
        _ => {
            let mut dest = Vec::with_capacity(origin.len());
            for item in origin {
                dest.push(serde_json::to_value(item.from_model()?)?)
            }
            Ok(Some(Value::Array(dest)))
        }
    }
}

pub fn from_lax_array<T: ModelConv>(origin: Option<Value>) -> Result<Vec<T>, Box<dyn Error>> {
    match origin {
        None => Ok(vec![]),
        Some(origin) => {
            if origin.is_array() {
                let inter: Vec<T::JsonSerdeValue> = serde_json::from_value(origin)?;
                let mut dest = Vec::with_capacity(inter.len());
                for item in inter {
                    dest.push(T::to_model(item)?);
                }
                Ok(dest)
            } else {
                Ok(vec![T::to_model(serde_json::from_value(origin)?)?])
            }
        }
    }
}

pub fn from_model_opt<T: ModelConv>(
    origin: Option<&T>,
) -> Result<Option<T::JsonSerdeValue>, Box<dyn Error>> {
    match origin {
        None => Ok(None),
        Some(origin) => Ok(Some(origin.from_model()?)),
    }
}

pub fn to_model_opt<T: ModelConv>(
    origin: Option<T::JsonSerdeValue>,
) -> Result<Option<T>, Box<dyn Error>> {
    match origin {
        None => Ok(None),
        Some(origin) => Ok(Some(T::to_model(origin)?)),
    }
}

pub fn boxed_from_model_opt<T: ModelConv>(
    origin: Option<&Box<T>>,
) -> Result<Option<Box<T::JsonSerdeValue>>, Box<dyn Error>> {
    match origin {
        None => Ok(None),
        Some(origin) => Ok(Some(Box::new(origin.from_model()?))),
    }
}

pub fn boxed_to_model_opt<T: ModelConv>(
    origin: Option<Box<T::JsonSerdeValue>>,
) -> Result<Option<Box<T>>, Box<dyn Error>> {
    match origin {
        None => Ok(None),
        Some(origin) => Ok(Some(Box::new(T::to_model(*origin)?))),
    }
}

/**
 * Schema: https://www.w3.org/TR/json-ld/#the-context
 */
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Context {
    Single(Iri),
    Mix(Vec<Context>),
    TermDefs(HashMap<String, Iri>),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(untagged)]
pub enum Iri {
    Direct(String),
    TypeCoercion(TypeCoercion),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct TypeCoercion {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    typ: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Object {
    #[serde(rename = "@context")]
    schema_context: Option<Context>,
    id: Option<String>,
    #[serde(rename = "type")]
    typ: Option<Value>,

    // https://www.w3.org/ns/activitystreams#Object
    attachment: Option<Value>,
    #[serde(rename = "attributeTo")]
    attributed_to: Option<Value>,
    audience: Option<Value>,
    bcc: Option<Value>,
    bto: Option<Value>,
    cc: Option<Value>,
    context: Option<Value>,
    generator: Option<Value>,
    // Range: Image | Link
    icon: Option<Value>,
    // Range: Image | Link
    image: Option<Value>,
    #[serde(rename = "inReplyTo")]
    in_reply_to: Option<Value>,
    location: Option<Value>,
    preview: Option<Value>,
    // Range: Collection
    replies: Option<Box<Object>>,
    tag: Option<Value>,
    to: Option<Value>,
    url: Option<Value>,
    content: Option<Value>,
    #[serde(rename = "contentMap")]
    content_map: Option<HashMap<String, String>>,
    name: Option<Value>,
    #[serde(rename = "nameMap")]
    name_map: Option<HashMap<String, String>>,
    duration: Option<String>,
    #[serde(rename = "mediaType")]
    media_type: Option<Value>,
    #[serde(rename = "endTime")]
    end_time: Option<String>,
    published: Option<String>,
    summary: Option<Value>,
    #[serde(rename = "summaryMap")]
    summary_map: Option<HashMap<String, String>>,
    updated: Option<String>,
    describes: Option<Box<Object>>,

    // https://www.w3.org/ns/activitystreams#Actor
    inbox: Option<String>,
    outbox: Option<String>,
    following: Option<String>,
    followers: Option<String>,
    #[serde(rename = "preferredUsername")]
    preferred_username: Option<String>,
    endpoints: Option<HashMap<String, String>>,

    // https://www.w3.org/ns/activitystreams#Activity
    actor: Option<Value>,
    instrument: Option<Value>,
    origin: Option<Value>,
    object: Option<Value>,
    result: Option<Value>,
    target: Option<Value>,

    // https://www.w3.org/ns/activitystreams#Collection
    #[serde(rename = "totalItems")]
    total_items: Option<usize>,
    // Range: CollectionPage | Link
    current: Option<Box<ObjectOrLink>>,
    // Range: CollectionPage | Link
    first: Option<Box<ObjectOrLink>>,
    // Range: CollectionPage | Link
    last: Option<Box<ObjectOrLink>>,
    items: Option<Value>,

    // https://www.w3.org/ns/activitystreams#OrderedCollection
    #[serde(rename = "orderedItems")]
    ordered_items: Option<Value>,

    // https://www.w3.org/ns/activitystreams#CollectionPage
    next: Option<Box<ObjectOrLink>>,
    prev: Option<Box<ObjectOrLink>>,
    // Range: Link | Collection
    #[serde(rename = "partOf")]
    part_of: Option<Box<ObjectOrLink>>,

    // https://www.w3.org/ns/activitystreams#OrderedCollectionPage
    #[serde(rename = "startIndex")]
    start_index: Option<usize>,

    // https://www.w3.org/ns/activitystreams#Relationship
    subject: Option<Box<ObjectOrLink>>,
    relationship: Option<Value>,

    // https://www.w3.org/ns/activitystreams#Tombstone
    former_type: Option<Value>,
    deleted: Option<String>,

    // https://www.w3.org/ns/activitystreams#Question
    #[serde(rename = "oneOf")]
    one_of: Option<Value>,
    #[serde(rename = "anyOf")]
    any_of: Option<Value>,
    closed: Option<Value>,

    // https://www.w3.org/ns/activitystreams#Place
    accuracy: Option<f64>,
    altitude: Option<f64>,
    latitute: Option<f64>,
    longitute: Option<f64>,
    radius: Option<f64>,
    units: Option<String>,

    // https://docs.joinmastodon.org/spec/activitypub/#as
    #[serde(rename = "manuallyApprovesFollowers")]
    manually_approves_followers: Option<bool>,
    #[serde(rename = "alsoKnownAs")]
    also_known_as: Option<Value>,
    #[serde(rename = "movedTo")]
    moved_to: Option<String>,
    sensitive: Option<bool>,

    // http://joinmastodon.org/ns#featured
    featured: Option<String>,

    // http://joinmastodon.org/ns#featuredTags
    #[serde(rename = "featuredTags")]
    featured_tags: Option<String>,

    // http://joinmastodon.org/ns#discoverable
    discoverable: Option<bool>,

    // http://joinmastodon.org/ns#suspended
    suspended: Option<bool>,

    // http://joinmastodon.org/ns#devices
    devices: Option<String>,

    // https://w3id.org/security/v1
    #[serde(rename = "publicKey")]
    public_key: Option<Key>,

    // https://schema.org/PropertyValue
    value: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Link {
    schema_context: Option<Context>,
    id: Option<String>,
    typ: Option<Value>,

    // https://www.w3.org/ns/activitystreams#Link
    href: String,
    height: Option<usize>,
    hreflang: Option<String>,
    media_type: Option<Value>,
    rel: Option<Value>,
    width: Option<usize>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum ObjectOrLink {
    Uri(String),
    Link(Link),
    Object(Object),
}

/**
 * Reference: https://w3c.github.io/vc-data-integrity/vocab/security/vocabulary.html#Key
 */
#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Key {
    id: String,
    owner: String,
    #[serde(rename = "publicKeyPem")]
    public_key_pem: Option<String>,
}

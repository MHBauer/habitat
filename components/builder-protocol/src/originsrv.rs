// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::fmt;
use std::result;

use hab_core;
use hab_core::package::Identifiable;

use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

pub use message::originsrv::*;
use message::{Persistable, Routable};
use sharding::InstaId;

impl Persistable for Origin {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Routable for OriginGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        // JW TODO: This won't accurately find the origin without it. We can switch to using the ID
        // of the origin or perform a reverse lookup by storing the name->ID map on a particular
        // originsrv server.
        Some(self.get_name().to_string())
    }
}

impl Routable for OriginCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

// AJ TODO: This is the wrong key here - we need to route this request to the origin
impl Routable for OriginInvitationValidateRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_invite_id()))
    }
}

impl Serialize for Origin {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin", 4));
        try!(strukt.serialize_field("id", &self.get_id()));
        try!(strukt.serialize_field("name", self.get_name()));
        try!(strukt.serialize_field("owner_id", &self.get_owner_id()));
        try!(strukt.serialize_field("private_key_name", self.get_private_key_name()));
        strukt.end()
    }
}

impl Routable for OriginMemberRemove {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginMemberListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Persistable for OriginSecretKey {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Serialize for OriginSecretKey {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_secret_key", 6));
        try!(strukt.serialize_field("id", &self.get_id()));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id()));
        try!(strukt.serialize_field("name", self.get_name()));
        try!(strukt.serialize_field("revision", self.get_revision()));
        try!(strukt.serialize_field("body", self.get_body()));
        try!(strukt.serialize_field("owner_id", &self.get_owner_id()));
        strukt.end()
    }
}

impl Routable for OriginSecretKeyCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl Routable for OriginSecretKeyGet {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl Persistable for OriginPublicKey {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Serialize for OriginPublicKey {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_public_key", 6));
        try!(strukt.serialize_field("id", &self.get_id()));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id()));
        try!(strukt.serialize_field("name", self.get_name()));
        try!(strukt.serialize_field("revision", self.get_revision()));
        try!(strukt.serialize_field("body", self.get_body()));
        try!(strukt.serialize_field("owner_id", &self.get_owner_id()));
        strukt.end()
    }
}

impl Routable for OriginPublicKeyCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl Routable for OriginPublicKeyGet {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl Routable for OriginPublicKeyLatestGet {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl Routable for OriginPublicKeyListRequest {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Routable for OriginPublicKeyListResponse {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_origin_id()))
    }
}

impl Serialize for OriginPublicKeyListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_public_key_list_response", 2));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id()));
        try!(strukt.serialize_field("keys", self.get_keys()));
        strukt.end()
    }
}

impl Routable for OriginInvitationCreate {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_owner_id())
    }
}

impl Persistable for OriginInvitation {
    type Key = u64;

    fn primary_key(&self) -> Self::Key {
        self.get_id()
    }

    fn set_primary_key(&mut self, value: Self::Key) {
        self.set_id(value);
    }
}

impl Serialize for OriginInvitation {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_invitation", 6));
        try!(strukt.serialize_field("id", &self.get_id()));
        try!(strukt.serialize_field("account_id", &self.get_account_id()));
        try!(strukt.serialize_field("accaccount_name", self.get_account_name()));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id()));
        try!(strukt.serialize_field("origin_name", self.get_origin_name()));
        try!(strukt.serialize_field("owner_id", &self.get_owner_id()));
        strukt.end()
    }
}

impl Routable for AccountInvitationListRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_account_id())
    }
}

impl Routable for AccountInvitationListResponse {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_account_id())
    }
}

impl Serialize for AccountInvitationListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("account_invitation_list_response", 2));
        try!(strukt.serialize_field("account_id", &self.get_account_id()));
        try!(strukt.serialize_field("invitations", self.get_invitations()));
        strukt.end()
    }
}

impl Routable for OriginInvitationListRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin_id())
    }
}

impl Routable for OriginInvitationListResponse {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_origin_id())
    }
}

impl Serialize for OriginInvitationListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_invitation_list_response", 2));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id()));
        try!(strukt.serialize_field("invitations", self.get_invitations()));
        strukt.end()
    }
}

impl Routable for OriginInvitationAcceptRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        // TODO: we don't have an origin id here...
        Some(self.get_invite_id())
    }
}

impl Serialize for OriginMemberListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_member_list_response", 2));
        try!(strukt.serialize_field("origin_id", &self.get_origin_id()));
        try!(strukt.serialize_field("members", self.get_members()));
        strukt.end()
    }
}

impl Routable for AccountOriginListRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_account_id())
    }
}

impl Serialize for AccountOriginListResponse {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("account_origin_list_response", 2));
        try!(strukt.serialize_field("account_id", &self.get_account_id()));
        try!(strukt.serialize_field("origins", self.get_origins()));
        strukt.end()
    }
}

impl Routable for CheckOriginAccessRequest {
    type H = u64;

    fn route_key(&self) -> Option<Self::H> {
        Some(self.get_account_id())
    }
}

impl Routable for OriginProjectGet {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_name()))
    }
}

impl Routable for OriginProjectCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_project().get_origin_id()))
    }
}

impl Routable for OriginProjectUpdate {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_project().get_origin_name()))
    }
}

impl Routable for OriginProjectDelete {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_name()))
    }
}

impl Serialize for OriginProject {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = try!(serializer.serialize_struct("project", 2));
        try!(state.serialize_field("id", &self.get_id()));
        try!(state.serialize_field("origin_id", &self.get_origin_id()));
        try!(state.serialize_field("origin_name", self.get_origin_name()));
        try!(state.serialize_field("package_name", self.get_package_name()));
        try!(state.serialize_field("name", self.get_name()));
        try!(state.serialize_field("plan_path", self.get_plan_path()));
        try!(state.serialize_field("owner_id", &self.get_owner_id()));
        try!(state.serialize_field("vcs_type", self.get_vcs_type()));
        try!(state.serialize_field("vcs_data", self.get_vcs_data()));
        state.end()
    }
}

impl Identifiable for OriginPackageIdent {
    fn origin(&self) -> &str {
        self.get_origin()
    }

    fn name(&self) -> &str {
        self.get_name()
    }

    fn version(&self) -> Option<&str> {
        let ver = self.get_version();
        if ver.is_empty() { None } else { Some(ver) }
    }

    fn release(&self) -> Option<&str> {
        let rel = self.get_release();
        if rel.is_empty() { None } else { Some(rel) }
    }
}

impl fmt::Display for OriginPackageIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.get_version().is_empty() && !self.get_release().is_empty() {
            write!(f,
                   "{}/{}/{}/{}",
                   self.get_origin(),
                   self.get_name(),
                   self.get_version(),
                   self.get_release())
        } else if !self.get_version().is_empty() {
            write!(f,
                   "{}/{}/{}",
                   self.get_origin(),
                   self.get_name(),
                   self.get_version())
        } else {
            write!(f, "{}/{}", self.get_origin(), self.get_name())
        }
    }
}

impl Into<hab_core::package::PackageIdent> for OriginPackageIdent {
    fn into(self) -> hab_core::package::PackageIdent {
        hab_core::package::PackageIdent::new(self.get_origin(),
                                   self.get_name(),
                                   Some(self.get_version()),
                                   Some(self.get_release()))
    }
}

impl Routable for OriginPackageGet {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl Routable for OriginPackageCreate {
    type H = InstaId;

    fn route_key(&self) -> Option<Self::H> {
        Some(InstaId(self.get_owner_id()))
    }
}

impl Routable for OriginPackageUniqueListRequest {
    type H = String;

    fn route_key(&self) -> Option<Self::H> {
        Some(String::from(self.get_origin()))
    }
}

impl Serialize for OriginPackageIdent {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut strukt = try!(serializer.serialize_struct("origin_package_ident", 4));
        try!(strukt.serialize_field("origin", self.get_origin()));
        try!(strukt.serialize_field("name", self.get_name()));
        if !self.get_version().is_empty() {
            try!(strukt.serialize_field("version", self.get_version()));
        }
        if !self.get_release().is_empty() {
            try!(strukt.serialize_field("release", self.get_release()));
        }
        strukt.end()
    }
}

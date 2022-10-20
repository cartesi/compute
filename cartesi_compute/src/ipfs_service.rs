// Dispatcher provides the infrastructure to support the development of DApps,
// mediating the communication between on-chain and off-chain components.

// Copyright (C) 2019 Cartesi Pte. Ltd.

// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.

// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Note: This component currently has dependencies that are licensed under the
// GNU GPL, version 3, and so you should treat this component as a whole as
// being under the GPL version 3. But all Cartesi-written code in this component
// is licensed under the Apache License, version 2, or a compatible permissive
// license, and can be used independently under the Apache v2 license. After
// this component is rewritten, the entire component will be released under the
// Apache v2 license.

//! A collection of types that represent the manager grpc interface
//! together with the conversion functions from the automatically
//! generated types.

use super::ethereum_types::H256;
use super::grpc::marshall::Marshaller;
use super::ipfs_interface::ipfs;

pub const IPFS_SERVICE_NAME: &'static str = "ipfs";
pub const IPFS_METHOD_GET: &'static str = "/CartesiIpfs.Ipfs/GetFile";
pub const IPFS_METHOD_ADD: &'static str = "/CartesiIpfs.Ipfs/AddFile";

/// Representation of a request for get file
#[derive(Debug, Clone)]
pub struct GetFileRequest {
    pub ipfs_path: String,
    pub log2_size: u32,
    pub output_path: String,
    pub timeout: u64,
}

/// Representation of the response of getting file
#[derive(Debug, Clone)]
pub struct GetFileResponse {
    pub one_of: GetFileResponseOneOf,
}

#[derive(Debug, Clone)]
pub enum GetFileResponseOneOf {
    GetProgress(Progress),
    GetResult(GetFileResult),
}

#[derive(Debug, Clone)]
pub struct Progress {
    pub progress: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone)]
pub struct GetFileResult {
    pub output_path: String,
    pub root_hash: H256,
}

impl From<ipfs::GetFileResponse_oneof_get_oneof> for GetFileResponseOneOf {
    fn from(one_of: ipfs::GetFileResponse_oneof_get_oneof) -> Self {
        match one_of {
            ipfs::GetFileResponse_oneof_get_oneof::progress(s) => {
                GetFileResponseOneOf::GetProgress(s.into())
            }
            ipfs::GetFileResponse_oneof_get_oneof::result(p) => {
                GetFileResponseOneOf::GetResult(p.into())
            }
        }
    }
}

impl From<ipfs::Progress> for Progress {
    fn from(progress: ipfs::Progress) -> Self {
        Progress {
            progress: progress.progress,
            updated_at: progress.updated_at,
        }
    }
}

impl From<ipfs::GetFileResult> for GetFileResult {
    fn from(result: ipfs::GetFileResult) -> Self {
        GetFileResult {
            output_path: result.output_path,
            root_hash: H256::from_slice(
                &result
                    .root_hash
                    .into_option()
                    .expect("root hash not found")
                    .data,
            ),
        }
    }
}

impl From<ipfs::GetFileResponse> for GetFileResponse {
    fn from(response: ipfs::GetFileResponse) -> Self {
        GetFileResponse {
            one_of: response.get_oneof.unwrap().into(),
        }
    }
}

/// Representation of a request for add file
#[derive(Debug, Clone)]
pub struct AddFileRequest {
    pub file_path: String,
}

/// Representation of the response of adding file
#[derive(Debug, Clone)]
pub struct AddFileResponse {
    pub one_of: AddFileResponseOneOf,
}

#[derive(Debug, Clone)]
pub enum AddFileResponseOneOf {
    AddProgress(Progress),
    AddResult(AddFileResult),
}

#[derive(Debug, Clone)]
pub struct AddFileResult {
    pub ipfs_path: String,
}

impl From<ipfs::AddFileResponse_oneof_add_oneof> for AddFileResponseOneOf {
    fn from(one_of: ipfs::AddFileResponse_oneof_add_oneof) -> Self {
        match one_of {
            ipfs::AddFileResponse_oneof_add_oneof::progress(s) => {
                AddFileResponseOneOf::AddProgress(s.into())
            }
            ipfs::AddFileResponse_oneof_add_oneof::result(p) => {
                AddFileResponseOneOf::AddResult(p.into())
            }
        }
    }
}

impl From<ipfs::AddFileResult> for AddFileResult {
    fn from(result: ipfs::AddFileResult) -> Self {
        AddFileResult {
            ipfs_path: result.ipfs_path,
        }
    }
}

impl From<ipfs::AddFileResponse> for AddFileResponse {
    fn from(response: ipfs::AddFileResponse) -> Self {
        AddFileResponse {
            one_of: response.add_oneof.unwrap().into(),
        }
    }
}

impl From<Vec<u8>> for GetFileResponse {
    fn from(response: Vec<u8>) -> Self {
        let marshaller: Box<
            dyn Marshaller<ipfs::GetFileResponse> + Sync + Send,
        > = Box::new(grpc::protobuf::MarshallerProtobuf);
        marshaller
            .read(bytes::Bytes::from(response))
            .unwrap()
            .into()
    }
}

impl From<Vec<u8>> for AddFileResponse {
    fn from(response: Vec<u8>) -> Self {
        let marshaller: Box<
            dyn Marshaller<ipfs::AddFileResponse> + Sync + Send,
        > = Box::new(grpc::protobuf::MarshallerProtobuf);
        marshaller
            .read(bytes::Bytes::from(response))
            .unwrap()
            .into()
    }
}

impl From<GetFileRequest> for Vec<u8> {
    fn from(request: GetFileRequest) -> Self {
        let marshaller: Box<
            dyn Marshaller<ipfs::GetFileRequest> + Sync + Send,
        > = Box::new(grpc::protobuf::MarshallerProtobuf);

        let mut req = ipfs::GetFileRequest::new();
        req.set_ipfs_path(request.ipfs_path);
        req.set_log2_size(request.log2_size);
        req.set_output_path(request.output_path);
        req.set_timeout(request.timeout);

        marshaller.write(&req).unwrap()
    }
}

impl From<AddFileRequest> for Vec<u8> {
    fn from(request: AddFileRequest) -> Self {
        let marshaller: Box<
            dyn Marshaller<ipfs::AddFileRequest> + Sync + Send,
        > = Box::new(grpc::protobuf::MarshallerProtobuf);

        let mut req = ipfs::AddFileRequest::new();
        req.set_file_path(request.file_path);

        marshaller.write(&req).unwrap()
    }
}

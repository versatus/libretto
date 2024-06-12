use tonic::{Request, Response, Status};
use crate::dfs::dfs_service_server::DfsService;
use crate::dfs::{StoreRequest, StoreResponse, LaunchRequest, LaunchResponse, HeartbeatRequest, HeartbeatResponse, ReplicateRequest, ReplicateResponse};

#[derive(Default)]
pub struct DfsRpcService;

#[tonic::async_trait]
impl DfsService for DfsRpcService {
    async fn store(
        &self,
        _request: Request<tonic::Streaming<StoreRequest>>,
    ) -> Result<Response<StoreResponse>, Status> {
        Ok(Response::new(StoreResponse { success: true }))
    }

    async fn launch(
        &self,
        _request: Request<LaunchRequest>,
    ) -> Result<Response<LaunchResponse>, Status> {
        Ok(Response::new(LaunchResponse { success: true }))
    }

    async fn heartbeat(
        &self,
        _request: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        Ok(Response::new(HeartbeatResponse { success: true }))
    }
    
    async fn replicate(
        &self,
        _request: Request<tonic::Streaming<ReplicateRequest>>
    ) -> Result<Response<ReplicateResponse>, Status> {
        Ok(Response::new(ReplicateResponse { success: true}))
    }
}

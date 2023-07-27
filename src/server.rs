use tonic::{transport::Server, Request, Response, Status};

use scheduler::scheduling_service_server::{SchedulingService, SchedulingServiceServer};
use scheduler::{SchedulingRequest, SchedulingResponse};

pub mod scheduler {
    tonic::include_proto!("orkascheduler"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MySchedulingService {}

#[tonic::async_trait]
impl SchedulingService for MySchedulingService {
    async fn schedule(
        &self,
        request: Request<SchedulingRequest>,
    ) -> Result<Response<SchedulingResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = SchedulingResponse {
            status_code: 0,
            rejection_reason: Some(1),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let scheduler = MySchedulingService::default();

    Server::builder()
        .add_service(SchedulingServiceServer::new(scheduler))
        .serve(addr)
        .await?;

    Ok(())
}

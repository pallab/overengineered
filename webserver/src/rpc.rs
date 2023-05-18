
pub mod rpc {
    // use tonic::{Request, Response, Status};
    use tonic::transport::{Channel, Server, Error};
    use crate::file_server::{ListFilesRequest, ListFilesResponse};
    use crate::file_server::files_client::FilesClient;

    pub async fn new_client(host: &str, port : u16) -> Result<FilesClient<Channel>, Error> {
        FilesClient::connect( format!("http://{host}:{port}")).await
    }

    pub async fn list_files(client : &mut FilesClient<Channel>) -> ListFilesResponse {
        let request = tonic::Request::new(
            ListFilesRequest { }
        );

        let response = client.list_files(request).await.expect("");
        println!("grpc response {:?}", response);

        response.into_inner()
    }
}
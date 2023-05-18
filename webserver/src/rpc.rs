
pub mod rpc {
    use tonic::IntoRequest;
    // use tonic::{Request, Response, Status};
    use tonic::transport::{Channel, Server, Error};
    use crate::file_server::{ListFilesRequest, ListFilesResponse, LoadFileRequest};
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

    pub async fn load_file_to_str(client : &mut FilesClient<Channel>, file_name : String) -> Result<String, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(
            LoadFileRequest { name : file_name }
        );

        let response = client.load_file(request).await.expect("");
        println!("grpc response {:?}", response);

        let mut stream = response.into_inner();

        let mut response = String::new();

        while let Some(msg) = stream.message().await? {
            println!("{}", msg.is_success);
            response.push_str(msg.is_success.to_string().as_str());
        }

        Ok(response)
    }
}
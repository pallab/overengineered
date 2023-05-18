
pub mod rpc {

    use tonic::transport::{Channel, Server, Error};
    use crate::stocks_rpc::{ListStocksRequest, ListStocksResponse, StockPriceRequest};
    use crate::stocks_rpc::stock_market_client::StockMarketClient;

    pub async fn new_client(host: &str, port : u16) -> Result<StockMarketClient<Channel>, Error> {
        StockMarketClient::connect( format!("http://{host}:{port}")).await
    }

    pub async fn list_stocks(client : &mut StockMarketClient<Channel>) -> ListStocksResponse {
        let request = tonic::Request::new(
            ListStocksRequest { }
        );

        let response = client.list_stocks(request).await.expect("");
        println!("grpc response {:?}", response);

        response.into_inner()
    }

    pub async fn get_price_ticks(client : &mut StockMarketClient<Channel>, file_name : String) -> Result<String, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(
            StockPriceRequest { name : file_name }
        );

        let response = client.get_stock_price(request).await.expect("");
        println!("grpc response {:?}", response);

        let mut stream = response.into_inner();

        let mut response = String::new();

        while let Some(msg) = stream.message().await? {
            println!("{:?}", msg);
            response.push_str(format!("{:?}\n", msg).as_str());
        }

        Ok(response)
    }
}
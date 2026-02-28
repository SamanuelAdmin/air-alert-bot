pub trait View {
    async fn show(&mut self, message: &str)
        -> Result<(), Box<dyn std::error::Error>>; 
}


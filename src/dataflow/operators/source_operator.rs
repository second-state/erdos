use std::thread;
use std::time::Duration;
use std::sync::{Arc};
use rand::{thread_rng, Rng};
use crate::{
    dataflow::message::{Timestamp, Message},
    dataflow::{Operator, OperatorConfig, WriteStream, stream::WriteStreamT},
    OperatorId,
};

#[allow(dead_code)]
pub struct SourceOperator {
    name: String,
    id: OperatorId,
    write_stream: Arc<WriteStream<usize>>,
}

impl Operator for SourceOperator {
    fn run(&mut self) {
        let mut ws = self.write_stream.clone();
        thread::spawn(move || {
            let one_sec = Duration::from_millis(1000);
            let mut i = 0;
            let mut rng = thread_rng();
            loop {
                i = i + 1;
                match Arc::get_mut(&mut ws) {
                    Some(ws) => {
                        let t = Timestamp::new(vec![i]);
                        let m = Message::<usize>::new_message(t, rng.gen::<u8>().into());
                        ws.send(m);
                        let t = Timestamp::new(vec![i]);
                        let m = Message::<usize>::new_watermark(t);
                        ws.send(m);
                    },
                    _ => ()
                }
                thread::sleep(one_sec);
            }
        });
    }
}

impl SourceOperator {
    #[allow(dead_code)]
    pub fn new(config: OperatorConfig<()>, write_stream: WriteStream<usize>) -> Self {
        Self {
            name: config.name.unwrap(),
            id: config.id,
            write_stream: Arc::new(write_stream),
        }
    }

    #[allow(dead_code)]
    pub fn connect() -> WriteStream<usize> {
        WriteStream::new()
    }

}

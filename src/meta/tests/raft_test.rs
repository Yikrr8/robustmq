#[cfg(test)]
mod tests {
    use byteorder::{BigEndian, ReadBytesExt};
    use common::config::meta::MetaConfig;
    use common::log;
    use meta::Meta;
    use prost::Message;
    use std::io::Cursor;
    use std::thread::sleep;
    use std::time::Duration;
    use std::vec;
    use raft::eraftpb::{
        ConfChange, ConfChangeType, Entry, EntryType, Message as raftPreludeMessage, Snapshot,
    };

    #[test]
    fn raft_node_1() {
        let mut conf = MetaConfig::default();
        conf.node_id = 1;
        conf.addr = "127.0.0.1".to_string();
        conf.port = 1221;
        conf.admin_port=2221;
        conf.log_path = "/tmp/test_fold1/logs".to_string();
        conf.data_path = "/tmp/test_fold1/data".to_string();
        conf.meta_nodes = vec![
            "127.0.0.1:1221".to_string(),
            "127.0.0.1:1222".to_string(),
            "127.0.0.1:1223".to_string(),
        ];

        log::new(conf.log_path.clone(), 1024, 50);

        let mut mt = Meta::new(conf);
        mt.start();

        loop {
            sleep(Duration::from_secs(1000));
        }
    }

    #[test]
    fn raft_node_2() {
        let mut conf = MetaConfig::default();
        conf.node_id = 2;
        conf.addr = "127.0.0.1".to_string();
        conf.port = 1222;
        conf.admin_port=2222;
        conf.log_path = "/tmp/test_fold2/logs".to_string();
        conf.data_path = "/tmp/test_fold2/data".to_string();
        conf.meta_nodes = vec![
            "127.0.0.1:1221".to_string(),
            "127.0.0.1:1222".to_string(),
            "127.0.0.1:1223".to_string(),
        ];

        log::new(conf.log_path.clone(), 1024, 50);

        let mut mt = Meta::new(conf);
        mt.start();

        loop {
            sleep(Duration::from_secs(1000));
        }
    }

    #[test]
    fn raft_node_3() {
        let mut conf = MetaConfig::default();
        conf.node_id = 3;
        conf.addr = "127.0.0.1".to_string();
        conf.port = 1223;
        conf.admin_port=2223;
        conf.log_path = "/tmp/test_fold3/logs".to_string();
        conf.data_path = "/tmp/test_fold3/data".to_string();
        conf.meta_nodes = vec![
            "127.0.0.1:1221".to_string(),
            "127.0.0.1:1222".to_string(),
            "127.0.0.1:1223".to_string(),
        ];

        log::new(conf.log_path.clone(), 1024, 50);

        let mut mt = Meta::new(conf);
        mt.start();

        loop {
            sleep(Duration::from_secs(1000));
        }
    }

    #[test]
    fn vec_test() {
        let v = vec![1, 2, 3, 4, 5, 6];
        let start = 0 as usize;
        let end = 3 as usize;
        println!("{:?}", v[start..end].to_vec());
        println!("{:?}", v[start..end].to_vec());
    }

    #[test]
    fn byte_order_test() {
        let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
        let v = rdr.read_u16::<BigEndian>().unwrap();
        println!("{}", v);

        // let mut wtr = vec![];
        // wtr.write_u16::<LittleEndian>(64).unwrap();
        // let mut rdr = Cursor::new(wtr);
        // let v = rdr.read_u64::<BigEndian>().unwrap();
        // println!("{}", v);

        let v1 = "666".to_string().into_bytes();
        println!("{:?}", v1);
        println!("{:?}", String::from_utf8(v1).unwrap());

        let v2 = 666u64.encode_to_vec();

        let v2 = 666u64.to_be_bytes();
        println!("{}",u64::from_be_bytes(v2));
    }

    #[test]
    fn entry_vec_test(){
        let mut entries = Vec::new();
        let mut e1 = Entry::default();
        e1.set_index(1);  
        entries.push(e1);

        let mut e2 = Entry::default();
        e2.set_index(2);
        entries.push(e2);


        let mut res_entries = Vec::new();
        res_entries.extend_from_slice(&entries);
        println!("{:?}",res_entries);

        res_entries.drain(1..);
        println!("{:?}",res_entries);

        let mut entries = Vec::new();
        let mut e1 = Entry::default();
        e1.set_index(2);  
        entries.push(e1);

        let mut e2 = Entry::default();
        e2.set_index(4);
        entries.push(e2);
        res_entries.extend_from_slice(&entries);
        println!("{:?}",res_entries);


    }
}
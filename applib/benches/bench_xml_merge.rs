use std::fs::File;
use std::io::Read;

use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

use applib::xmlutil::{read_file, read_text_in_node};
use mockito::ServerGuard;
use applib::get_hotel_accomodations_from_suppliers;

use applib::{create_hash, simulate_cpu};

async fn get_supplier_url_mock(server: &mut ServerGuard, supplier: &str, buf: &[u8]) -> mockito::Mock {
    let supplierpath = format!("/{}", supplier);
    let _m = server.mock("GET", supplierpath.as_str())
    .with_status(200)        
    .with_body(buf)
    .create_async().await;
    _m
}

async fn get_supplier_data(files: Vec<&str>) -> (Vec<String>, Vec<Vec<u8>>){

    let mut suppliers = Vec::new();
    let mut data = Vec::new();

    for (i, file) in files.iter().enumerate() {
        let xml_buf = read_file(file).await.unwrap();
        suppliers.push(i.to_string());
        data.push(xml_buf)
    }
    (suppliers, data)
}

fn supplier_combine_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("supplier_data");
   

    group.bench_function("supplier_xml_merge", |bench| {
        bench.iter_with_setup(
            || {
                let rt = Runtime::new().unwrap();

                let (suppliers, xml_data) =
                rt.block_on(get_supplier_data(vec![
                        "xmls/sup_1_10001.xml",
                        "xmls/sup_2_10001.xml",
                        "xmls/sup_3_10001.xml",
                        "xmls/sup_4_10001.xml"
                    ]));

                let mut server = rt.block_on(mockito::Server::new_async());
                let url = server.url();

                let _m0 = rt.block_on(get_supplier_url_mock(&mut server, suppliers.get(0).unwrap(), xml_data.get(0).unwrap()));
                let _m1 = rt.block_on(get_supplier_url_mock(&mut server, suppliers.get(1).unwrap(), xml_data.get(1).unwrap()));
                let _m2 = rt.block_on(get_supplier_url_mock(&mut server, suppliers.get(2).unwrap(), xml_data.get(2).unwrap()));
                let _m3 = rt.block_on(get_supplier_url_mock(&mut server, suppliers.get(3).unwrap(), xml_data.get(3).unwrap()));
                (rt, suppliers, url)
            },
            |(rt, suppliers, url)| {
                rt.block_on(get_hotel_accomodations_from_suppliers(suppliers, &url)).unwrap();
            },
        );
    });

    group.finish();
}

// fn criterion_benchmark2(c: &mut Criterion) {
//     let rt = Runtime::new().unwrap();

//     let (suppliers, xml_data) =
//     rt.block_on(get_supplier_data(vec![
//             "../xmls/sup_1_10001.xml",
//             "../xmls/sup_1_10002.xml",
//             "../xmls/sup_1_10003.xml",
//             "../xmls/sup_1_10004.xml"
//         ]));

//     let mut server = rt.block_on(mockito::Server::new_async());
//     let url = server.url();

//     let _m0 = rt.block_on(get_supplier_url_mock(&mut server, suppliers.get(0).unwrap(), xml_data.get(0).unwrap()));
//     let _m1 = rt.block_on(get_supplier_url_mock(&mut server, suppliers.get(1).unwrap(), xml_data.get(1).unwrap()));
    
//     let xml_buf = rt.block_on(read_file("../xmls/sup_1_10001.xml")).unwrap();
//     let xml_data = String::from_utf8(xml_buf).unwrap();
//     let node_name = "Hotels".to_string();

//     c.bench_function("read_text_in_node", |b| {
//         b.iter(|| {
//             rt.block_on(get_accomodations_fromSuppliers(suppliers, &url)).unwrap();
//         });
//     });
// }
fn read_file_to_string(file_path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

fn benchmark_simple_hash(c: &mut Criterion) {
    // Sample data to hash
    
    let data = read_file_to_string("xmls/sup_7_10001.xml").expect("Failed to read file");
    let data = String::from_utf8(data).unwrap();

    // Benchmark the simple_hash function
    c.bench_function("simple_hash", |b| {
        b.iter(|| simulate_cpu(&data, 31, 30))
    });
}


criterion_group!(benches, supplier_combine_benchmark,benchmark_simple_hash);
criterion_main!(benches);

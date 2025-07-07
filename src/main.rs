// #![no_std]
slint::include_modules!();
use serialport;
use slint::{ComponentHandle, platform};
use sysinfo::{ System, Disks, Networks, Users };

fn main() {
    let env = platform::Platform.get();
    let app = MainWindow::new().unwrap();
    let weak = app.as_weak();
    app.global::<AppAction>().on_change_text(move |str| {
        let app: MainWindow = weak.unwrap();
        app.set_value(str + "set_value".into())
    });
    app.window().set_maximized(true);
    app.window().set_size(env.primary_screen());
    app.run().unwrap();

    // 查找可用的串口
    let ports = serialport::available_ports().unwrap();

    // 选择要连接的串口（这里假设只有一个可用串口，你可能需要根据实际情况进行选择）
    // let port = ports[0].port;
    println!("{:#?}----------", ports);

    let networks = Networks::new_with_refreshed_list();
    let disks = Disks::new_with_refreshed_list();
    let sys_info = System::new_all();
    let users = Users::new_with_refreshed_list();

    println!("MAC address: {:#?}", networks);
    println!("{:?}",sys_info);
    println!("{:?}", users.len());
    println!("{:?}", disks);


    // loop {}
    // 打开串口
    // let mut serial = port.open().unwrap();
    // 设置串口参数（波特率、数据位、停止位、奇偶校验等）
    // serial.set_baud_rate(9600).unwrap();
    // serial.set_data_bits(serialport::DataBits::Eight).unwrap();
    // serial.set_stop_bits(serialport::StopBits::One).unwrap();
    // serial.set_parity(serialport::Parity::None).unwrap();


}
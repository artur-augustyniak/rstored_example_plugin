#[no_mangle]
pub fn hosts() -> String {
    let ret = r#"{ "file-name":"etc/hosts", "len":31, "content":"[aaugustyniak@bender somelib]$ cat /etc/hosts
127.0.0.1   localhost localhost.localdomain localhost4 localhost4.localdomain4 bender
::1         localhost localhost.localdomain localhost6 localhost6.localdomain6 bender

#VIRT
192.168.122.3   docker-host
192.168.122.4   valkyrie
" }"#.to_string();
ret
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

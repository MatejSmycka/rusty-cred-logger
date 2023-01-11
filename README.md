# muni-is-mirror

## Setup

```
pip3 install -r requirements.txt
sudo mv systemd-unit/phishing.service /etc/systemd/system/phishing.service 
sudo systemctl enable phishing.service
sudo systemctl start phishing.service
```

## Start

```
cargo run
```


Thus, when configuring these values in Hermes' `config.toml`, keep in mind that this is how these 
parameters will be used. If the total clock drift is too small, then we run the risk of client
updates being rejected because a new block won't have been created yet. It's better to err on the
side of total clock drift being larger than smaller, however, if this value ends up being _too_
large, then this becomes a security vulnerability.
CONFIG_PATH=${HOME}/.oxyqueue

.PHONY: init

init:
	mkdir -p ${CONFIG_PATH}

.PHONY: gencert

gencert:
	cfssl gencert -initca ca-csr.json | cfssljson -bare ca
	cfssl gencert \
		-ca=ca.pem \
		-ca-key=ca-key.pem \
		-config=ca-config.json \
		-profile=server \
		server-csr.json | cfssljson -bare server
	mv *.pem *.csr ${CONFIG_PATH}

SHELL:=/bin/bash
prefix = /usr/local
exec_prefix = $(prefix)
sbindir = $(exec_prefix)/bin
libdir = $(exec_prefix)/lib
logdir = /var/log

templogger:
	cargo build -r

install: templogger
	sudo mkdir -p -m=777 $(logdir)/templogger
	sudo chown -R $(USER) $(logdir)/templogger
	sudo install -Dm755 target/release/temploggerd $(sbindir)/temploggerd;
	sudo install -Dm755 target/release/temp $(sbindir)/temp;
	sudo install -Dm644 templogger.service $(libdir)/systemd/system/templogger.service;

uninstall: clean
	sudo rm -f $(sbindir)/temploggerd;
	sudo rm -f $(sbindir)/temp;
	sudo rm -f $(libdir)/systemd/system/templogger.service;

clean:
	@echo -n "Would you like to remove the logs? (y/n) " && read ans && if [ $${ans:-'n'} = 'y' ]; \
	then \
		echo "sudo rm -rf $(logdir)/templogger"; \
		sudo rm -rf $(logdir)/templogger; \
		exit 0; \
	else \
		exit 0; \
	fi

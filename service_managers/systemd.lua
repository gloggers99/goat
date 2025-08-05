-- systemd has an interesting specificaation.
binary_name = "systemctl"
hostname_reload_command = "hostnamectl set-hostname \"$(cat /etc/hostname)\""
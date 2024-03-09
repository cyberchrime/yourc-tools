# Reverse Engineering the UR-C Tools

## Setup

First, you must setup a virtual machine with a windows guest.
You can download a Windows installation ISO from Microsoft.
You may use any virtualizer of your choice. In case you would
like to give QEMU a try, you can follow this guide.

Create a virtual hard drive of 50G size with

```
qemu-img create -f qcow2 disk.img 50G
```

Afterwards, install Windows onto that disk:

```
qemu-system-x86_64 -enable-kvm -m 4G -hda disk.img -nic none
```

Note that the previous command runs the VM without a Network Card.
Once the installation is done, you may shutdown the VM and restart
it again with the last option removed to have internet access

```
qemu-system-x86_64 -enable-kvm -m 4G -hda disk.img

```

Once your Windows is set up, download and install the UR-C tools
from Steinberg's website.

Afterwards, detect the USB Bus and Device ID with `lsusb`, e.g.

```
$ lsusb
Bus 003 Device 004: ID 0499:1730 Yamaha Corp. Steinberg UR44C
```

In this example, the bus ID is `3` and the device ID `4`. Connect
them to your VM with

```
sudo qemu-system-x86_64 -enable-kvm -m 4G -hda disk.img -device qemu-xhci -device usb-host,hostbus=3,hostaddr=4
```

You may use the parameter `pcap=ur44c.pcap` to record the usb traffic, e.g.

```
sudo qemu-system-x86_64 -enable-kvm -m 4G -hda disk.img -device qemu-xhci -device usb-host,id=ur44,hostbus=3,hostaddr=4,pcap=ur44c.pcap
```

When using the option `-monitor stdio`, you can hotplug your audio interface to verify if settings applied or not:

```
sudo qemu-system-x86_64 -enable-kvm -m 4G -hda disk.img -device qemu-xhci -device usb-host,id=ur44,hostbus=3,hostaddr=4,pcap=ur44c.pcap
(qemu) device_del ur44c
(qemu) device_add usb-host,id=ur44c,hostbus=3,hostaddr=3,pcap=ur44c2.pcap
```



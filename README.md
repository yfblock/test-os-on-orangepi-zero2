# 测试在 Orangepi Zero2 上写 OS

## 工具需求

> 为了方便开发，本项目使用 sunxi-tools 来进行“烧写”，可以实现不用手动重启烧录和启动。

- sunxi-tools
- pyserial
- rust

## 如何使用

### 1. 烧录 sd 卡 (在某些情况下可以不需要这一步)

> 这一步需要保证 sd 卡已经分区。

``` shell
sudo dd if=fel-sdboot.sunxi of=/dev/mmcblk0 bs=8k seek=1
make sdcard
```
### 2. 插上开发板, 烧录 os

> 在某些版本的 sunxi-tools 中可能存在问题，需要自行在 github 下载源码编译

```
make flash
```

### 更新一下进度

2023-5-17: 手上这块orangepi zero2  好像坏了，等待下单再卖一块

查询资料所得内存分布图：0x40000000-0x1_3FFF_FFFF 4G
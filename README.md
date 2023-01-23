# pi

1. mpfr pi定数 ・・・・・・・・・・・・・・・・・・・・・ [pi_cons](pi_cons)
2. mpfr arctan(1)  ・・・・・・・・・・・・・・・・・・・・ [pi_atan](pi_atan)
3. 算術幾何平均 (AGM ; Arithmetic and Geometric Mean) ・・ [pi_agm](pi_agm)

    [2003 Hironobu SUZUKI]　(リンク切れ)(2003)　[suzuki](suzuki)

4. Machin's formula  ・・・・・・・・・・・・・・・・・・・ [machin](machin)
5. Chudnovsky Binary Splitting Method ・・・・・・・・・・

    [円周率を1億桁計算しました！ － その試行錯誤の詳しい経緯と結果 －](https://itchyny.hatenablog.com/entry/20120304/1330870932)(2014)　[itchyny](itchyny)
    
    [Rustを使って円周率1億桁計算したお話](https://zenn.dev/uu/articles/48e2d4098b6aca)(2021)　[yapatta](yapatta)
    
    [Chudnovsky の公式を用いた円周率の計算用メモ](https://qiita.com/peria/items/c02ef9fc18fb0362fb89)

6. Million Digit Pi Benchmark

すずきひろのぶ氏の円周率１００万桁ソースコードを用いて各世代RaspberryPiの速度比較を行う。

|Hardware|Central<br>Processing<br>Unit|Operating<br>System|gcc|gmp|time|
|---|---|---|---|---|---:|
|Raspberry Pi 4<br>Model B Rev 1.2<br>(2019)|ARM Cortex-A72<br>1.5GHz|RaspberryPiOS<br>(Debian 11 (bullseye))|10.2.1|6.2.1|	4.875s|
|Raspberry Pi 3<br>Model B Rev 1.2<br>(2016)|ARM Cortex-A53<br>1.2GHz|RaspberryPiOS<br>(Debian 11 (bullseye))|10.2.1|6.2.1|13.826s|
|NanoPi NEO<br>(2016)|Allwinner H3<br>(Quad-core Cortex-A7)<br>1.2GHz|Ubuntu 16.04.7 LTS|5.4.0|6.2.1|21.383s|
|Raspberry Pi 2<br>Model B Rev 1.1<br>(2015)|ARM Cortex-A7<br>900MHz|RaspberryPiOS<br>(Debian 11 (bullseye))|10.2.1|6.2.1|21.798s|
|DevTerm R-01<br>(2022)|RISC-V 64bit Single-core<br>1.0GHz<br>RV64IMAFDCVU|Ubuntu 22.04.1 LTS|11.3.0|6.2.1|32.788s|
|Raspberry Pi Zero<br>Model B Rev 2<br>(2018)|ARM1176JZF-S<br>1GHz|RaspberryPiOS<br>(Debian 11 (bullseye))|10.2.1|6.2.1|33.960s|
|Raspberry Pi<br>Model B Rev 2<br>(2012)|ARM1176JZF-S<br>700MHz|RaspberryPiOS<br>(Debian 11 (bullseye))|10.2.1|6.2.1|48.573s|

左上から raspberry pi (初代)、raspberry pi 2。中段左から raspberry pi 3、raspberry pi 4。下段左から raspberry pi Zero、NanoPi NEO。
![raspberry-pi.jpg](raspberry-pi.jpg)
![devterm-r01.jpg](devterm-r01.jpg)




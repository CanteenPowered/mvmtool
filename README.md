## vtabledump - EA command-line tool for querying Valve MvM servers 

## Building
1. Install the [Rust toolchain](https://rustup.rs/)
2. Open the project folder from a terminal
3. Run `cargo install --path .` to install
4. Obtain a Steam web API key here: https://steamcommunity.com/dev/apikey
5. Run `mvmtool sey-key <key>`

## Usage
```
USAGE:
    mvmtool <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    find             Finds MvM matches
    help             Prints this message or the help of the given subcommand(s)
    list-missions    Lists known missions
    set-key          Sets the Steam web API key
```
<details>
  <summary>Example: Searching for active Mecha Engine matches with `mvmtool find -t me`</summary>
  
  ```
Valve Matchmaking Server (Washington srcds210-tuk2 #86)
    IP: 169.254.85.177:8536
    Mission: Broken Parts (mvm_bigrock_advanced1)
    Players: 6/6
        Admiral Glufo
            Kills: 28
            Time: 28m31s
        that one medic
            Kills: 46
            Time: 28m31s
        sniping5000
            Kills: 58
            Time: 28m29s
        Patty
            Kills: 165
            Time: 28m27s
        pink
            Kills: 26
            Time: 28m21s
        Jettzky
            Kills: 0
            Time: 1m55s
Valve Matchmaking Server (Madrid srcds1015-mad1 #104)
    IP: 169.254.210.37:46992
    Mission: Bone Shaker (mvm_bigrock_advanced2)
    Players: 6/6
        DURAGON FAIL KNIFEX
            Kills: 68
            Time: 16m28s
        missing_texture
            Kills: 122
            Time: 16m28s
        loshok3785
            Kills: 48
            Time: 16m28s
        getting up to mischief
            Kills: 97
            Time: 16m28s
        dubious little creature
            Kills: 11
            Time: 16m26s
        eug | mashalLah
            Kills: 86
            Time: 16m21s
Valve Matchmaking Server (Stockholm srcds8158-sto1 #42)
    IP: 169.254.244.181:19600
    Mission: Disintegration (mvm_decoy_advanced3)
    Players: 6/6
        Wapolocaust
            Kills: 85
            Time: 24m25s
        Spottylime96
            Kills: 23
            Time: 24m25s
        GGToys
            Kills: 47
            Time: 24m25s
        Tal_dimant
            Kills: 72
            Time: 24m25s
        Leonardo808
            Kills: 51
            Time: 24m25s
        rx792086
            Kills: 4
            Time: 3m40s
=======================================================
Total servers: 3
Total players: 18
Total open slots: 0
=======================================================
  ```
  
</details>

<details>
  <summary>Example: Querying all active Valve MvM servers with `mvmtool find --skip-players`</summary>
  
  ```
Valve Matchmaking Server (Virginia srcds1015-iad1 #44)
    IP: 169.254.198.216:64400
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Washington srcds110-tuk2 #23)
    IP: 169.254.190.4:18736
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds1015-iad1 #236)
    IP: 169.254.163.241:14472
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #181)
    IP: 169.254.206.237:61960
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #100)
    IP: 169.254.209.80:58896
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 3/6
Valve Matchmaking Server (Frankfurt srcds201-fra2 #78)
    IP: 169.254.88.188:20064
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (LA srcds1008-lax1 #90)
    IP: 169.254.213.237:59856
    Mission: Crash Course (mvm_coaltown)
    Players: 1/6
Valve Matchmaking Server (Virginia srcds1015-iad1 #33)
    IP: 169.254.158.231:46928
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Washington srcds110-tuk2 #47)
    IP: 169.254.233.82:33880
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 6/6
Valve Matchmaking Server (Washington srcds210-tuk2 #24)
    IP: 169.254.113.249:31744
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Tokyo srcds2154-tyo2 #16)
    IP: 169.254.127.224:59464
    Mission: Ctrl+Alt+Destruction (mvm_coaltown_advanced)
    Players: 3/6
Valve Matchmaking Server (Virginia srcds1015-iad1 #55)
    IP: 169.254.134.18:56952
    Mission: Mann-euvers (mvm_mannworks)
    Players: 1/6
Valve Matchmaking Server (Frankfurt srcds201-fra2 #147)
    IP: 169.254.207.54:47960
    Mission: Crash Course (mvm_coaltown)
    Players: 6/6
Valve Matchmaking Server (Singapore srcds3030-sgp2 #305)
    IP: 169.254.46.221:59864
    Mission: Machine Massacre (mvm_mannworks_advanced)
    Players: 5/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #196)
    IP: 169.254.148.103:62120
    Mission: Crash Course (mvm_coaltown)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds2015-iad1 #22)
    IP: 169.254.116.232:33376
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Tokyo srcds2154-tyo2 #34)
    IP: 169.254.163.141:31160
    Mission: Machine Massacre (mvm_mannworks_advanced)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds2015-iad1 #102)
    IP: 169.254.86.110:27024
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 2/6
Valve Matchmaking Server (Virginia srcds1015-iad1 #64)
    IP: 169.254.207.40:39656
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Washington srcds110-tuk2 #21)
    IP: 169.254.63.153:36792
    Mission: Cataclysm (mvm_coaltown_expert1)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #12)
    IP: 169.254.203.69:52456
    Mission: Doe's Drill (mvm_decoy)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #26)
    IP: 169.254.158.13:17552
    Mission: Cave-in (mvm_coaltown_intermediate)
    Players: 1/6
Valve Matchmaking Server (Madrid srcds1014-mad1 #19)
    IP: 169.254.170.133:33152
    Mission: Machine Massacre (mvm_mannworks_advanced)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds2015-iad1 #42)
    IP: 169.254.77.56:44464
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds201-fra2 #168)
    IP: 169.254.152.109:11232
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 6/6
Valve Matchmaking Server (Singapore srcds3030-sgp2 #216)
    IP: 169.254.72.106:35736
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Sydney srcds2011-syd1 #40)
    IP: 169.254.90.109:52864
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Washington srcds210-tuk2 #72)
    IP: 169.254.137.233:11464
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 1/6
Valve Matchmaking Server (Virginia srcds2015-iad1 #73)
    IP: 169.254.40.97:9720
    Mission: Desperation (mvm_decoy_expert1)
    Players: 6/6
Valve Matchmaking Server (Washington srcds210-tuk2 #86)
    IP: 169.254.85.177:8536
    Mission: Broken Parts (mvm_bigrock_advanced1)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #161)
    IP: 169.254.74.188:23656
    Players: 6/6
Valve Matchmaking Server (Madrid srcds1015-mad1 #104)
    IP: 169.254.210.37:46992
    Mission: Bone Shaker (mvm_bigrock_advanced2)
    Players: 6/6
Valve Matchmaking Server (LA srcds1009-lax1 #97)
    IP: 169.254.177.132:35624
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds1015-iad1 #148)
    IP: 169.254.168.194:53528
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds201-fra2 #92)
    IP: 169.254.133.104:2936
    Mission: Doe's Doom (mvm_decoy_intermediate)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #172)
    IP: 169.254.181.165:64640
    Mission: Village Vanguard (mvm_rottenburg)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds1015-iad1 #166)
    IP: 169.254.244.172:49688
    Mission: Bavarian Botbash (mvm_rottenburg_advanced2)
    Players: 6/6
Valve Matchmaking Server (Sydney srcds1012-syd1 #96)
    IP: 169.254.203.90:53800
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #39)
    IP: 169.254.169.128:61576
    Mission: Crash Course (mvm_coaltown)
    Players: 3/6
Valve Matchmaking Server (Singapore srcds3030-sgp2 #382)
    IP: 169.254.51.245:60752
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds2015-iad1 #173)
    IP: 169.254.46.172:26840
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Sydney srcds1013-syd1 #370)
    IP: 169.254.201.203:55208
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds201-fra2 #163)
    IP: 169.254.116.147:30640
    Mission: Bavarian Botbash (mvm_rottenburg_advanced2)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds2015-iad1 #198)
    IP: 169.254.57.157:26880
    Mission: Mannslaughter (mvm_mannworks_expert1)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds1015-iad1 #234)
    IP: 169.254.106.49:22136
    Mission: Desperation (mvm_decoy_expert1)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds2015-iad1 #219)
    IP: 169.254.141.73:23480
    Mission: Bavarian Botbash (mvm_rottenburg_advanced2)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds2015-iad1 #223)
    IP: 169.254.242.195:36784
    Mission: Ctrl+Alt+Destruction (mvm_coaltown_advanced)
    Players: 5/6
Valve Matchmaking Server (Virginia srcds2015-iad1 #227)
    IP: 169.254.24.132:58648
    Players: 1/6
Valve Matchmaking Server (Frankfurt srcds201-fra2 #228)
    IP: 169.254.133.37:39784
    Mission: Doe's Drill (mvm_decoy)
    Players: 6/6
Valve Matchmaking Server (LA srcds1009-lax1 #89)
    IP: 169.254.149.181:1960
    Mission: Bavarian Botbash (mvm_rottenburg_advanced2)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #234)
    IP: 169.254.103.80:46920
    Players: 6/6
Valve Matchmaking Server (Sydney srcds2011-syd1 #345)
    IP: 169.254.69.205:15224
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #129)
    IP: 169.254.13.237:11360
    Mission: Cave-in (mvm_coaltown_intermediate)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds201-fra2 #171)
    IP: 169.254.205.231:13864
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Singapore srcds2030-sgp2 #242)
    IP: 169.254.172.21:29648
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 1/6
Valve Matchmaking Server (Sydney srcds2012-syd1 #341)
    IP: 169.254.205.67:34608
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Singapore srcds3030-sgp2 #347)
    IP: 169.254.66.191:65320
    Mission: Disk Deletion (mvm_decoy_advanced)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #73)
    IP: 169.254.246.254:32024
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #83)
    IP: 169.254.9.52:19752
    Mission: Bavarian Botbash (mvm_rottenburg_advanced2)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds201-fra2 #40)
    IP: 169.254.32.132:22832
    Mission: Cave-in (mvm_coaltown_intermediate)
    Players: 6/6
Valve Matchmaking Server (Frankfurt srcds101-fra2 #134)
    IP: 169.254.235.126:17240
    Mission: Doe's Drill (mvm_decoy)
    Players: 5/6
Valve Matchmaking Server (LA srcds1008-lax1 #4)
    IP: 169.254.124.70:19880
    Mission: Crash Course (mvm_coaltown)
    Players: 6/6
Valve Matchmaking Server (Washington srcds110-tuk2 #98)
    IP: 169.254.15.239:6720
    Mission: Cataclysm (mvm_coaltown_expert1)
    Players: 6/6
Valve Matchmaking Server (LA srcds1009-lax1 #8)
    IP: 169.254.137.122:31672
    Mission: Village Vanguard (mvm_rottenburg)
    Players: 6/6
Valve Matchmaking Server (LA srcds1009-lax1 #18)
    IP: 169.254.171.82:28168
    Mission: Caliginous Caper (mvm_ghost_town)
    Players: 6/6
Valve Matchmaking Server (LA srcds1009-lax1 #74)
    IP: 169.254.226.5:49632
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Virginia srcds2015-iad1 #65)
    IP: 169.254.156.20:38352
    Mission: Doe's Doom (mvm_decoy_intermediate)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds4158-sto1 #12)
    IP: 169.254.67.98:16112
    Mission: Hamlet Hostility (mvm_rottenburg_advanced1)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds8158-sto1 #27)
    IP: 169.254.230.76:39856
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds5158-sto1 #29)
    IP: 169.254.48.225:35752
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds2158-sto1 #9)
    IP: 169.254.59.34:32560
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds5158-sto1 #48)
    IP: 169.254.110.2:25480
    Mission: Crash Course (mvm_coaltown)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds4158-sto1 #4)
    IP: 169.254.52.81:31656
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds2158-sto1 #42)
    IP: 169.254.126.82:11288
    Mission: Crash Course (mvm_coaltown)
    Players: 5/6
Valve Matchmaking Server (Stockholm srcds4158-sto1 #39)
    IP: 169.254.181.251:55760
    Mission: Caliginous Caper (mvm_ghost_town)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds4158-sto1 #43)
    IP: 169.254.196.28:51152
    Mission: Bavarian Botbash (mvm_rottenburg_advanced2)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds6158-sto1 #12)
    IP: 169.254.21.176:7072
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds8158-sto1 #42)
    IP: 169.254.244.181:19600
    Mission: Disintegration (mvm_decoy_advanced3)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds8158-sto1 #29)
    IP: 169.254.135.251:40864
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds2158-sto1 #48)
    IP: 169.254.122.39:55968
    Mission: Empire Escalation (mvm_mannhattan_advanced1)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds8158-sto1 #60)
    IP: 169.254.229.90:57512
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Stockholm srcds8158-sto1 #26)
    IP: 169.254.240.135:26336
    Mission: Bavarian Botbash (mvm_rottenburg_advanced2)
    Players: 6/6
Valve Matchmaking Server (Hong Kong srcds1074-hkg1 #43)
    IP: 169.254.58.95:15416
    Mission: Caliginous Caper (mvm_ghost_town)
    Players: 6/6
Valve Matchmaking Server (Hong Kong srcds1074-hkg1 #48)
    IP: 169.254.104.84:14176
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Hong Kong srcds1074-hkg1 #32)
    IP: 169.254.160.247:2904
    Mission: Caliginous Caper (mvm_ghost_town)
    Players: 6/6
Valve Matchmaking Server (Hong Kong srcds2074-hkg1 #60)
    IP: 169.254.124.119:56120
    Mission: Ctrl+Alt+Destruction (mvm_coaltown_advanced)
    Players: 5/6
Valve Matchmaking Server (Hong Kong srcds2074-hkg1 #17)
    IP: 169.254.173.211:53376
    Players: 6/6
Valve Matchmaking Server (Hong Kong srcds2074-hkg1 #23)
    IP: 169.254.69.23:56680
    Mission: Mann-euvers (mvm_mannworks)
    Players: 3/6
Valve Matchmaking Server (Hong Kong srcds2074-hkg1 #25)
    IP: 169.254.174.175:56184
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 6/6
Valve Matchmaking Server (Hong Kong srcds2074-hkg1 #40)
    IP: 169.254.40.246:24360
    Mission: Metro Malice (mvm_mannhattan_advanced2)
    Players: 1/6
Valve Matchmaking Server (Hong Kong srcds2074-hkg1 #46)
    IP: 169.254.45.254:49800
    Mission: Village Vanguard (mvm_rottenburg)
    Players: 4/6
========================================================
Total servers: 91
Total players: 488
Total open slots: 58
========================================================
  ```
  
</details>

## Contributing

All contributions are welcome. If you find a bug or want to request a feature, please [create an issue](https://github.com/CanteenPowered/mvmtool/issues/new).

## License
[MIT](/LICENSE)

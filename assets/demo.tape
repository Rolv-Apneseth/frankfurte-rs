Output demo.gif

Require cargo

Set Shell "bash"
Set FontSize 32
Set Width 1200
Set Height 1440
Set FontFamily "Inconsolata Nerd Font Mono"
Set PlaybackSpeed 0.5

Type "frs currencies" Sleep 250ms  Enter
Sleep 1s

Type "frs convert EUR USD" Sleep 250ms  Enter
Sleep 1s

Type "frs convert GBP EUR,USD,SEK,AUD" Sleep 250ms  Enter
Sleep 1s

Type "frs convert BRL HKD,SGD,THB --amount 100" Sleep 250ms  Enter
Sleep 1s

Type "frs convert AUD PHP --date 2018-04-02" Sleep 250ms  Enter
Sleep 1s

Type "frs period EUR CAD --start 2025-01-01 --end 2025-02-01" Sleep 500ms Enter
Sleep 1s

Type "frs period SEK NZD,RON,HUF --start 2007-05-03 --end 2007-05-12 --json" Sleep 500ms Enter
Sleep 1s

Type "frs period EUR CHF,NOK,GBP --start 2000-01-01 --end 2000-02-01" Sleep 500ms Enter
Sleep 2s

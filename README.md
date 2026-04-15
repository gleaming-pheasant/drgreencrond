# drgreencrond
## Hello, my name is Dr. GreenCron
<p align="center">
    <img
        src="https://github.com/gleaming-pheasant/drgreencrond/resources/dgclogo-sml.png?raw=true"
        alt="the drgreencron logo" 
    />
</p>

## About
dtgreencrond is a task scheduling daemon similar to 
[cron](https://wiki.archlinux.org/title/Cron) or 
[systemd Timers](https://wiki.archlinux.org/title/Systemd/Timers).

It is a partial replacement for the *timer* part of `systemd Timers`. In the 
background it handles executing processes in the same way as `systemd Timers`. 
However, rather than scheduling on realtime or monotonic timers, it schedules 
daily, based on the UK's NESO Carbon Intensity API.

It is not a drop-in replacement for `cron` or `systemd Timers`. Because it 
cannot handle monotonic timers, or timers that must run at an exact time, those 
should continue to be scheduled as required.

Instead, tasks should be scheduled with this daemon if they simply need to be 
executed arbitrarily within a day/week/month.

Tasks can also be scheduled to run __NOT__ within a specific timeframe, for 
example, if you don't want a task running between business hours 
(`Mon..Fri 09:00..17:00`).
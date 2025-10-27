/// Interface with the SDV platform
import signal speed_ms : float;$\label{line:import:speed_ms}$
export signal os_alert : Alert;$\label{line:export:os_alert}$
/// Speed range type
enum Range { Low, Mid, High }$\label{line:type:Range}$
/// Alert type
enum Alert { None, Level1, Level2 }$\label{line:type:Alert}$
/// Over-Speed Alert service
service over_speed_alert @ [10, 5000] {$\label{line:service:osa:begin}$
  let signal speed_range: Range = get_range(speed_ms);$\label{line:service:osa:speed_range}$
  let event range_change: Range = on_change(speed_range);$\label{line:service:osa:range_change}$
  let event timer: unit = timeout(range_change, 1000);$\label{line:service:osa:timer}$
  os_alert = get_alert(speed_range, range_change, timer);$\label{line:service:osa:os_alert}$
}$\label{line:service:osa:end}$
/// Main over-speed component
component get_alert(range: Range, change: Range?, timer: unit?) -> (alert: Alert){$\label{line:component:get_alert:begin2}$
  alert = when {$\label{line:component:get_alert:alert:when:begin}$
    init => Alert::None,$\label{line:component:get_alert:alert:when:init}$
    // alert rising
    timer? if last alert == Alert::None$\label{line:component:get_alert:alert:when:timerlast}$
              && range == Range::Mid => Alert::Level1,
    timer? if last alert == Alert::None
              && range == Range::High => Alert::Level1,
    timer? if last alert == Alert::Level1
              && range == Range::High => Alert::Level2,
    // alert decrease
    change? if last alert == Alert::Level2
               && range == Range::Mid => Alert::Level1,
    change? if last alert == Alert::Level2
               && range == Range::Low => Alert::None,
    change? if last alert == Alert::Level1
               && range == Range::Low => Alert::None,
  };$\label{line:component:get_alert:alert:when:end}$
}$\label{line:component:get_alert:end}$
/// Speed range computation
function get_range(speed: float) -> Range {$\label{line:function:get_range:begin}$
  let range: Range = match speed {$\label{line:function:get_range:range:match:begin}$
    s if s > 33.0 => Range::High,
    s if s > 22.0 => Range::Mid,
    _             => Range::Low,$\label{line:function:get_range:range:match:wildcard}$
  };$\label{line:function:get_range:range:match:end}$
  return range;$\label{line:function:get_range:return}$
}$\label{line:function:get_range:end}$
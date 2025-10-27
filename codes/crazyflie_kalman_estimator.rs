/// Imports from sensors.
import signal acc      : [float; 3];
import signal gyro     : [float; 3];
import signal measure  : float;                                                                     $\label{line:kalman:import:meas}$
/// Reset request.
import event  reset_ev : unit;
/// Estimator's state made accessible.
export signal estimator_state : EstimatorState;                                                     $\label{line:kalman:export:state}$
/// Service performing the Kalman filter
service kalman_task @[1, 3000] {                                                                    $\label{line:kalman:param}$
    let event measure_ev : float = on_change(measure);                                              $\label{line:kalman:event:meas}$
    estimator_state = kalman(acc, gyro, measure_ev, reset_ev, time());
}
/// The main Kalman component.
component kalman(acc: [float; 3], gyro: [float; 3],
                 measure: float?, reset: unit?, t: float) -> (external: EstimatorState) {
    when {                                                                                          $\label{line:kalman:when:start}$
        init => { (predicted, noisy, updated) = (reset(), reset(), reset()); }
        reset? => {                                                                              $\label{line:kalman:when:reset}$
            let predicted: InternalState  = reset();
            let noisy: InternalState = noise(predicted, t);
            let updated: InternalState = noisy;
        }
        measure? => {                                                                            $\label{line:kalman:when:meas}$
            let predicted: InternalState = predict(last noisy, acc, gyro, t);                       $\label{line:kalman:when:meas:predict}$
            let noisy: InternalState = noise(predicted, t);
            let updated: InternalState  = update(noisy, measure);                                $\label{line:kalman:when:meas:update}$
        }
    }                                                                                               $\label{line:kalman:when:end}$
    let finalized: InternalState = finalize(updated);
    external = externalize(finalized);
}
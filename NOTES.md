Some notes on how to interpret MTA realtime data as it relates to schedule data

TripReplacementPeriod: defines a time range during which realtime data overrides schedule data.
- For certain routes this is now() + 1h, for others it is now(). 
- The period applies to what time a trip STARTS, e.g. if a trip started before the current time, realtime data overrides schedule data for it.
    - Do testing to determine if the realtime data should also apply to trips starting very soon

TripUpdate: defines an update to a scheduled trip.
- Gives a list of a handful of StopTimeUpdates, which define the arrival/departure time to a specific station for this trip

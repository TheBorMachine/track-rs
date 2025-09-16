stress-event:
	wrk -t12 -c400 -d30s -s tests/stress/event.lua http://localhost:8000/event

stress-events:
	wrk -t12 -c400 -d120s -s tests/stress/events.lua http://localhost:8000/events

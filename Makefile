stress-event:
	wrk -t4 -c100 -d120s -s tests/stress/event.lua http://localhost:8000/event

stress-events:
	wrk -t4 -c100 -d120s -s tests/stress/events.lua http://localhost:8000/events

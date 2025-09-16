wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

counter = 0

request = function()
    counter = counter + 1
    local body = string.format('{"session":"%d","event_type":"%d"}', counter, counter, math.random(18, 60))
    return wrk.format(nil, nil, nil, body)
end

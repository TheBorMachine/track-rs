wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"

counter = 0

request = function()
    counter = counter + 1

    local events = math.random(1, 20)
    local body = "["
    
    for i = 1, events do
        body = body .. string.format('{"session": "session_%d", "event_type": "click"}', math.random(1, 10000000))
        
        if i < events then
            body = body .. ","
        end
    end
    
    body = body .. "]"
    
    return wrk.format(nil, nil, nil, body)
end
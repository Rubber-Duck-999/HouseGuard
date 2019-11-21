package com;

import java.io.IOException;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.concurrent.TimeoutException;
import com.house_guard.Common.*;


import com.google.gson.Gson;
import com.rabbitmq.client.Channel;
import com.rabbitmq.client.Connection;
import com.rabbitmq.client.ConnectionFactory;
import com.rabbitmq.client.DeliverCallback;


public class ConsumerTopic
{
    private static final String EXCHANGE_NAME = "topics";
    private static ConnectionFactory factory;
    private static Connection connection;
    private static Channel channel;
    private String subscribeQueueName;
    private boolean accessAllowed;
    private boolean accessStateSet;
    private Integer receivedId;
    private Gson gson;
    
    public boolean getAccessState()
    {
    	return accessAllowed;
    }
    
    public boolean getAccessStateSet()
    {
    	return accessStateSet;
    }
    
    public void setAccessStateSetOff()
    {
    	accessStateSet = false;
    }
    
    public void sendMonitorState(boolean state)
    {
        String routingKey = Types.MONITOR_STATE_TOPIC;
        gson = new Gson();
        MonitorState mon = new MonitorState();
        mon.setState(state);
        String json = gson.toJson(mon);
        System.out.println("Message is : " + json);
        try 
        {
			channel.basicPublish(EXCHANGE_NAME, routingKey, null, json.getBytes());
		} 
        catch (IOException e) 
        {
			System.out.println("We have had issues publishing");
			e.printStackTrace();
		}   	
    }

    public void askForAccess(Integer key, Integer val)
    {
        String routingKey = Types.REQUEST_ACCESS_TOPIC;
        gson = new Gson();
        RequestAccess req = new RequestAccess();
        req.setId(key);
        req.setPin(val);
        String json = gson.toJson(req);
        System.out.println("Message is : " + json);
        try 
        {
			channel.basicPublish(EXCHANGE_NAME, routingKey, null, json.getBytes());
		} 
        catch (IOException e) 
        {
			System.out.println("We have had issues publishing");
			e.printStackTrace();
		}
    }
    
    private void accessResponse(String delivery, String routingKey)
    {
    	System.out.println(delivery);
        accessStateSet = true; 
		AccessResponse data = gson.fromJson(delivery, AccessResponse.class);
        receivedId = data.getId();        		
        if(!data.getResult().equals(Types.PASS))
        {
        	accessAllowed = false;
            String pubMessage = createEventUpMessage();
            try 
            {
				channel.basicPublish(EXCHANGE_NAME, Types.PUB_EVENT_TOPIC, null, pubMessage.getBytes());
			} 
            catch (IOException e) 
            {
				e.printStackTrace();
			}
        }
        else if(data.getResult().equals(Types.PASS))
        {
        	accessAllowed = true;
        }
    }
    
    private String createEventUpMessage() 
    {
        EventTopic user_event = new EventTopic();
        user_event.setComponent(Types.COMPONENT_NAME);
        user_event.setError(Types.RequestFailure);
        SimpleDateFormat formatter = new SimpleDateFormat("HH:mm:ss");  
        Date date = new Date();
        user_event.setTime(formatter.format(date));
        user_event.setSeverity(Types.ACCESS_NOT_RECEIVED);
        String pubMessage = gson.toJson(user_event);		
		return pubMessage;
	}

	public void consumeRequired()
    {
        try 
        {
        	Gson gson = new Gson();
			subscribeQueueName = channel.queueDeclare().getQueue();
	        channel.queueBind(subscribeQueueName, EXCHANGE_NAME, Types.ACCESS_RESPONSE);
	        System.out.println(" [*] Waiting for access.response. To exit press CTRL+C");
	        DeliverCallback deliverCallback = (consumerTag, delivery) -> {
	        	System.out.println("Message received");
	        	String received = new String(delivery.getBody());
	        	String key = delivery.getEnvelope().getRoutingKey();
	        	this.accessResponse(received, key);
	        };
	        channel.basicConsume(subscribeQueueName, true, deliverCallback, consumerTag -> { });
		} 
        catch (IOException e) 
        {
			e.printStackTrace();
		}
    }
    
    
    public ConsumerTopic()
    {
    	receivedId = 0;
    	accessAllowed = false;
    	accessStateSet = false;
        factory = new ConnectionFactory();
        factory.setHost("localhost");
        try 
        {
			connection = factory.newConnection();
			channel = connection.createChannel();
	        channel.exchangeDeclare(EXCHANGE_NAME, "topic");
		} 
        catch (IOException | TimeoutException e) 
        {
			System.out.println("We have had trouble setting up the required connection");
			e.printStackTrace();
		}
    }

	public Integer getId() 
	{
		return receivedId;		
	}
}

package com;

import java.io.IOException;
import java.util.concurrent.TimeoutException;

import com.google.gson.Gson;
import com.rabbitmq.client.Channel;
import com.rabbitmq.client.Connection;
import com.rabbitmq.client.ConnectionFactory;
import com.rabbitmq.client.DeliverCallback;


class ConsumerTopic
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

    public void AskForAccess(Integer key, Integer val)
    {
        String routingKey = Types.REQUEST_ACCESS_TOPIC;
        gson = new Gson();
        RequestAccess req = new RequestAccess();
        req.setId(key);
        req.setPin(val);
        String json = gson.toJson(req);
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
    
    private void AccessReponse(String delivery, String routingKey)
    {
        //System.out.println(" [x] Received '" + routingKey + "':'" + delivery + "'");
        accessStateSet = true; 
		AccessResponse data = gson.fromJson(delivery, AccessResponse.class);
        receivedId = data.getId();        		
        if(!data.getResult().equals("PASS"))
        {
        	accessAllowed = false;
            Event user_event = new Event();
            user_event.setComponent(Types.COMPONENT_NAME);
            user_event.setError("We have access denied");
            user_event.setTime("12:00:00");
            user_event.setSeverity(2);
            String pubMessage = gson.toJson(user_event);
            try 
            {
				channel.basicPublish(EXCHANGE_NAME, Types.PUB_EVENT_TOPIC, null, pubMessage.getBytes());
			} 
            catch (IOException e) 
            {
				e.printStackTrace();
			}
        }
        else if(data.getResult().equals("PASS"))
        {
        	accessAllowed = true;
        }
    }
    
    public void ConsumeRequired()
    {
        try 
        {
        	Gson gson = new Gson();
			subscribeQueueName = channel.queueDeclare().getQueue();
	        channel.queueBind(subscribeQueueName, EXCHANGE_NAME, "access.response");
	        System.out.println(" [*] Waiting for access.response. To exit press CTRL+C");
	        DeliverCallback deliverCallback = (consumerTag, delivery) -> {
	        	System.out.println("Message received");
	        	String received = new String(delivery.getBody());
	        	String key = delivery.getEnvelope().getRoutingKey();
	        	this.AccessReponse(received, key);
	        };
	        channel.basicConsume(subscribeQueueName, true, deliverCallback, consumerTag -> { });
		} 
        catch (IOException e) 
        {
			e.printStackTrace();
		}
    }
    
    
    ConsumerTopic()
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

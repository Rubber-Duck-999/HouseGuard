

import java.io.IOException;
import java.text.SimpleDateFormat;
import java.util.Date;
import java.util.concurrent.TimeoutException;

import com.google.gson.Gson;
import com.rabbitmq.client.Channel;
import com.rabbitmq.client.Connection;
import com.rabbitmq.client.ConnectionFactory;
import com.rabbitmq.client.DeliverCallback;

import com.house_guard.Common.*;


public class ConsumerTopic
{
    private static final String EXCHANGE_NAME = "topics";
    private static ConnectionFactory factory;
    private static Connection connection;
    private static Channel channel;
    private String subscribeQueueName;
    
    private void eventsTopicSubscribe(String delivery, String routingKey)
    {
    	
    }

	public void consumeRequired()
    {
        try 
        {
        	Gson gson = new Gson();
			subscribeQueueName = channel.queueDeclare().getQueue();
	        channel.queueBind(subscribeQueueName, EXCHANGE_NAME, Types.PUB_EVENT_TOPIC);
	        System.out.println(" [*] Waiting for access.response. To exit press CTRL+C");
	        DeliverCallback deliverCallback = (consumerTag, delivery) -> {
	        	System.out.println("Message received");
	        	String received = new String(delivery.getBody());
	        	String key = delivery.getEnvelope().getRoutingKey();
	        	this.eventsTopicSubscribe(received, key);
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

}

package com;

import com.rabbitmq.client.Channel;
import com.rabbitmq.client.Connection;
import com.rabbitmq.client.ConnectionFactory;
import com.rabbitmq.client.DeliverCallback;

import java.util.logging.Logger;

public class mainClass
{

    private static final String EXCHANGE_NAME = "topics";
    private static ConnectionFactory factory;
    private static Model myModel;
    private static View myView;
    private static Controller myController;

    public static void startUI()
    {
        int start_value = 10;
        myModel = new Model();
        myView = new View();
        
        MonitorView monitorView = new MonitorView();
        myController = new Controller(myModel, myView, monitorView);
        myController.initmodel(start_value, Types.OFF);
        myView.addController(myController);
        monitorView.addController(myController);
    }

    public static void main(String[] argv) throws Exception
    {
        startUI();
        factory = new ConnectionFactory();
        factory.setHost("localhost");
        Connection connection = factory.newConnection();
        Channel channel = connection.createChannel();

        channel.exchangeDeclare(EXCHANGE_NAME, "topic");
        String queueName = channel.queueDeclare().getQueue();

        String routingKey = "access.request";
        String message = "pin:1234\n";

        channel.basicPublish(EXCHANGE_NAME, routingKey, null, message.getBytes("UTF-8"));
        System.out.println(" [x] Sent '" + routingKey + "':'" + message + "'");

        channel.queueBind(queueName, EXCHANGE_NAME, "access.response");
        System.out.println(" [*] Waiting for access.response. To exit press CTRL+C");
        DeliverCallback deliverCallback = (consumerTag, delivery) -> {
            String received = new String(delivery.getBody(), "UTF-8");
            System.out.println(" [x] Received '" + delivery.getEnvelope().getRoutingKey() + "':'" + received + "'");
            if(!received.equals("PASS"))
            {
                String pubRoutingKey = "event.UP";
                //myModel.setMessageReceived(received);
                String pubMessage = "We have access denied at 14:00";
                channel.basicPublish(EXCHANGE_NAME, pubRoutingKey, null, pubMessage.getBytes("UTF-8"));
            }
            //System.exit(1);
        };
        channel.basicConsume(queueName, true, deliverCallback, consumerTag -> { });
    }
}

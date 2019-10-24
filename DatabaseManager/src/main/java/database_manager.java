import com.rabbitmq.client.Channel;
import com.rabbitmq.client.Connection;
import com.rabbitmq.client.ConnectionFactory;
import com.rabbitmq.client.DeliverCallback;


public class database_manager
{

  private static final String EXCHANGE_NAME = "topics";
  private static ConnectionFactory factory;

  public static void main(String[] argv) throws Exception
  {
      factory = new ConnectionFactory();
      factory.setHost("localhost");
      Connection connection = factory.newConnection();
      Channel channel = connection.createChannel();

      channel.exchangeDeclare(EXCHANGE_NAME, "topic");
      String queueName = channel.queueDeclare().getQueue();

      channel.queueBind(queueName, EXCHANGE_NAME, "event.*");
      System.out.println(" [*] Waiting for access.response. To exit press CTRL+C");
      DeliverCallback deliverCallback = (consumerTag, delivery) -> {
          String received = new String(delivery.getBody(), "UTF-8");
          System.out.println(" [x] Received '" + delivery.getEnvelope().getRoutingKey() + "':'" + received + "'");
          //System.exit(1);
      };
      channel.basicConsume(queueName, true, deliverCallback, consumerTag -> { });
  }
}

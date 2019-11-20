import java.util.logging.Logger;

public class mainClass
{
    private static ConsumerTopic cons;

    public static void start()
    {
        cons = new ConsumerTopic();
        cons.consumeRequired();
    }

    public static void main(String[] argv) throws Exception
    {
        start();
    }
}

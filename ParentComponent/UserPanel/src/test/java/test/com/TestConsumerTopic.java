package test.com;

import static org.junit.Assert.*;

import org.junit.Test;

import com.*;

public class TestConsumerTopic 
{

	@Test
	public void testGetAccessState() 
	{
		ConsumerTopic topic = new ConsumerTopic();
		assertFalse(topic.getAccessState());
	}

	@Test
	public void testGetAccessStateSet() 
	{
		ConsumerTopic topic = new ConsumerTopic();
		assertFalse(topic.getAccessStateSet());
	}

	@Test
	public void testSetAccessStateSetOff() 
	{
		ConsumerTopic topic = new ConsumerTopic();
		assertFalse(topic.getAccessState());
		topic.setAccessStateSetOff();
		assertFalse(topic.getAccessState());
	}

	@Test
	public void testAskForAccess() 
	{
		ConsumerTopic topic = new ConsumerTopic();
		topic.askForAccess(1, 1234);
		assertFalse(topic.getAccessState());
	}
	
	@Test
	public void testConsumeRequired() 
	{
		ConsumerTopic topic = new ConsumerTopic();
		topic.askForAccess(1, 1234);
		topic.consumeRequired();
		assertFalse(topic.getAccessState());
	}	
}

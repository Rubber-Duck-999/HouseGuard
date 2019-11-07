package test.com;

import static org.junit.Assert.*;

import org.junit.Test;

import com.*;

public class TestHarness 
{

	@Test
	public void testSubConsumer() 
	{
		StubConsumerTopic stub = new StubConsumerTopic();
        Model myModel = new Model();
        StubView myView = new StubView();
        StubMonitorView myMonitorView = new StubMonitorView();
        RequestTable table = new RequestTable();
		Controller myController = new Controller(myModel, myView, myMonitorView, stub, table);
        myController.initmodel(0, Types.OFF);
        myView.addController(myController);
        myMonitorView.addController(myController);
        stub.consumeRequired();
        
        // Simulates User clicking button 4 times
        myModel.incrementValue(0);
        myModel.incrementValue(1);
        myModel.incrementValue(2);
        myModel.incrementValue(3);
        Integer expected = 1111;
        Integer correctKey = 1;
        Integer incorrectKey = 5;
        //Expected 1111
        
        myController.enterCommand();
        assertTrue(table.doesKeyExist(correctKey));
        assertFalse(table.doesKeyExist(incorrectKey));
        assertTrue(table.doesValueExist(expected));
	}
	
	
	@Test
	public void TableCheckKey()
	{
		RequestTable table = new RequestTable();
		Integer pin = 1111;
		Integer key = table.addRecordNextKeyReturn(pin);
		Integer expected = 1;
		assertEquals(key, expected);
	}

	@Test
	public void TableCheckKeyThreeTimes()
	{
		RequestTable table = new RequestTable();
		Integer pin = 1111;
		table.addRecord(pin);
		table.addRecord(pin);
		Integer key = table.addRecordNextKeyReturn(pin);
		Integer expected = 3;
		assertEquals(key, expected);
	}
	
	@Test
	public void TableAddCrap()
	{
		RequestTable table = new RequestTable();
		Integer pin = 1111;
		table.addRecordNextKeyReturn(pin);
		table.addRecordNextKeyReturn(pin);
		Integer key = table.addRecordNextKeyReturn(pin);
		Integer expected = 3;
		assertTrue(table.doesValueExist(pin));
		assertEquals(key, expected);
	}
}

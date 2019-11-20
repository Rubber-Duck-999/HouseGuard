package test.com;

import static org.junit.Assert.*;

import java.awt.event.ActionEvent;
import com.house_guard.Common.*;
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
	public void testSubConsumer2() 
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
        myController.checkAccess();
        assertTrue(table.doesKeyExist(correctKey));
        assertFalse(table.doesKeyExist(incorrectKey));
        assertTrue(table.doesValueExist(expected));
	}
	
	@Test
	public void TestCheckActionPerformed()
	{
		StubConsumerTopic stub = new StubConsumerTopic();
        Model myModel = new Model();
        StubView myView = new StubView();
        StubMonitorView myMonitorView = new StubMonitorView();
        RequestTable table = new RequestTable();
		Controller myController = new Controller(myModel, myView, myMonitorView, stub, table);
		myController.checkAction(Types.Actions.ADD_D2.name());
		Integer expected = 100;
		assertEquals(myModel.checkPass(), expected);
	}

	@Test
	public void TestCheckActionPerformedMultiple()
	{
		StubConsumerTopic stub = new StubConsumerTopic();
        Model myModel = new Model();
        StubView myView = new StubView();
        StubMonitorView myMonitorView = new StubMonitorView();
        RequestTable table = new RequestTable();
		Controller myController = new Controller(myModel, myView, myMonitorView, stub, table);
		myController.checkAction(Types.Actions.ADD_D2.name());
		myController.checkAction(Types.Actions.ADD_D2.name());
		myController.checkAction(Types.Actions.ADD_D2.name());
		myController.checkAction(Types.Actions.ADD_D2.name());
		myController.checkAction(Types.Actions.ADD_D2.name());
		myController.checkAction(Types.Actions.ADD_D1.name());
		myController.checkAction(Types.Actions.ADD_D4.name());
		Integer expected = 1501;
		assertEquals(myModel.checkPass(), expected);
	}
	
	@Test
	public void TestCheckActionPerformedMultiple2()
	{
		StubConsumerTopic stub = new StubConsumerTopic();
        Model myModel = new Model();
        myModel.initModel(0);
        StubView myView = new StubView();
        StubMonitorView myMonitorView = new StubMonitorView();
        RequestTable table = new RequestTable();
		Controller myController = new Controller(myModel, myView, myMonitorView, stub, table);
		myController.checkAction(Types.Actions.ADD_D1.name());
		myController.checkAction(Types.Actions.ADD_D2.name());
		myController.checkAction(Types.Actions.ADD_D3.name());
		myController.checkAction(Types.Actions.ADD_D4.name());
		Integer expected_first = 1111;
		assertEquals(myModel.checkPass(), expected_first);
		//
		myController.checkAction(Types.Actions.SUB_D1.name());
		myController.checkAction(Types.Actions.SUB_D2.name());
		myController.checkAction(Types.Actions.SUB_D3.name());
		myController.checkAction(Types.Actions.SUB_D4.name());
		expected_first = 0;
		assertEquals(myModel.checkPass(), expected_first);
		//
		myController.checkAction(Types.Actions.SUB_D1.name());
		myController.checkAction(Types.Actions.SUB_D2.name());
		myController.checkAction(Types.Actions.SUB_D3.name());
		myController.checkAction(Types.Actions.SUB_D4.name());
		//
		myController.checkAction(Types.State.ON.name());
		myController.checkAction(Types.State.ON.name());
		myController.checkAction(Types.State.OFF.name());
		Integer expected = 9999;
		assertEquals(myModel.checkPass(), expected);
		assertNotEquals(myModel.setModelStateOFF(), Types.State.ON.name());
		assertEquals(myModel.setModelStateOFF(), Types.State.OFF.name());
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

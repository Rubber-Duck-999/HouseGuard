package test.com;

import static org.junit.Assert.*;

import org.junit.Test;

import com.*;
import com.house_guard.Common.EventTopic;

public class TestJsonTypes 
{

	@Test
	public void testGetId() 
	{
		AccessResponse access = new AccessResponse();
		Integer id = 100;
		access.setId(id);
		assertEquals(id, access.getId());
	}
	
	@Test
	public void testGetResult() 
	{
		AccessResponse access = new AccessResponse();
		String result = "FAIL";
		access.setResult(result);
		assertEquals(result, access.getResult());
	}
	
	@Test
	public void testGetErrorChecks() 
	{
		EventTopic up = new EventTopic();
		String component = "FAIL";
		up.setComponent(component);
		assertEquals(component, up.getComponent());
		//
		String error = "Fail to run";
		up.setError(error);
		assertEquals(error, up.getError());
		//
		String time = "14:00";
		up.setTime(time);
		assertEquals(time, up.getTime());
		//
		Integer severity = 5;
		up.setSeverity(severity);
		assertEquals(severity, up.getSeverity());
	}
		

}

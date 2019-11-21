package test.com;

import static org.junit.Assert.*;

import org.junit.Test;

import com.*;
import com.house_guard.Common.*;

public class TestTypes 
{

	@Test
	public void testActions() 
	{
		assertEquals(Types.Actions.ADD_D1.name(), "ADD_D1");
		assertEquals(Types.Actions.ADD_D2.name(), "ADD_D2");
		assertEquals(Types.Actions.ADD_D3.name(), "ADD_D3");
		assertEquals(Types.Actions.ADD_D4.name(), "ADD_D4");
		assertEquals(Types.Actions.SUB_D1.name(), "SUB_D1");
		assertEquals(Types.Actions.SUB_D2.name(), "SUB_D2");
		assertEquals(Types.Actions.SUB_D3.name(), "SUB_D3");
		assertEquals(Types.Actions.SUB_D4.name(), "SUB_D4");
		assertEquals(Types.State.ON.name(), "ON");
		assertEquals(Types.State.OFF.name(), "OFF");		
		
	}

}

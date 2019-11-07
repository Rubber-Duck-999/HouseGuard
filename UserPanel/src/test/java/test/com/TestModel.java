package test.com;

import static org.junit.Assert.*;

import org.junit.Test;

import com.*;

public class TestModel 
{

	@Test
	public void testConstructed() 
	{
		Model myModel = new Model();
		int TestPasscode = myModel.checkPass();
		assertEquals(0, TestPasscode);
	}

	@Test
	public void testInit() 
	{
		Model myModel = new Model();
		Integer TestBeforePasscode = 1;
		@SuppressWarnings("unused")
		Integer unneeded = myModel.initModel(TestBeforePasscode);
		Integer TestPasscode = myModel.checkPass();
		Integer expected = 1111;
		assertEquals(expected, TestPasscode);
	}
	
	@Test
	public void testIncrement()
	{
		Model myModel = new Model();
		Integer testBeforePasscode = 0;
		@SuppressWarnings("unused")
		Integer unneeded = myModel.initModel(testBeforePasscode);
		@SuppressWarnings("unused")
		Integer testPasscode = myModel.checkPass();
		Integer actual = myModel.incrementValue(0);
		Integer expected = 1;
		assertEquals(actual, expected);		
	}
	
	@Test
	public void testDecrement()
	{
		Model myModel = new Model();
		Integer testBeforePasscode = 0;
		@SuppressWarnings("unused")
		Integer unneeded = myModel.initModel(testBeforePasscode);
		@SuppressWarnings("unused")
		Integer testPasscode = myModel.checkPass();
		Integer actual = myModel.decrementValue(0);
		Integer expected = 9;
		assertEquals(actual, expected);		
	}
	
	@Test
	public void testIncrementFail()
	{
		Model myModel = new Model();
		Integer testBeforePasscode = 0;
		@SuppressWarnings("unused")
		Integer unneeded = myModel.initModel(testBeforePasscode);
		@SuppressWarnings("unused")
		Integer testPasscode = myModel.checkPass();
		Integer actual = myModel.incrementValue(0);
		Integer expected = 0;
		assertNotEquals(actual, expected);		
	}
	
	@Test
	public void testDecrementFail()
	{
		Model myModel = new Model();
		Integer testBeforePasscode = 0;
		@SuppressWarnings("unused")
		Integer unneeded = myModel.initModel(testBeforePasscode);
		@SuppressWarnings("unused")
		Integer testPasscode = myModel.checkPass();
		Integer actual = myModel.decrementValue(0);
		Integer expected = 6;
		assertNotEquals(actual, expected);		
	}
	
	@Test
	public void testDigitOutOfRange()
	{
		Model myModel = new Model();
		Integer testBeforePasscode = 9;
		@SuppressWarnings("unused")
		Integer unneeded = myModel.initModel(testBeforePasscode);
		@SuppressWarnings("unused")
		Integer testPasscode = myModel.checkPass();
		Integer actual = myModel.incrementValue(3);
		Integer expected = 6;
		assertNotEquals(actual, expected);			
	}

}

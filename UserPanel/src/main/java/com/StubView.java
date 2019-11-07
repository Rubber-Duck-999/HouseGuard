package com;

import java.awt.event.ActionListener;

public class StubView extends View
{

	public StubView()
	{
		
	}

	@Override
	public < E > void setDigit1(E v)
	{
		System.out.println(v);
	}

	@Override
	public < E > void setDigit2(E v)
	{
		
	}

	@Override
	public < E > void setDigit3(E v)
	{
		
	}

	@Override
	public < E > void setDigit4(E v)
	{
		
	}

	@Override
	public < E > void setDigits(E v)
	{
		
	}

	@Override
	public void addController(ActionListener listenerButtons)
	{
		System.out.println("addController call");
	}

	@Override
	public void displayErrorMessage(String message)
	{
		
	}

	@Override
	public void displayPassMessage(String message)
	{
		
	}

}

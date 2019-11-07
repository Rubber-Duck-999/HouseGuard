package com;

import java.awt.event.ActionListener;

public class StubMonitorView extends MonitorView
{
	boolean stubCall;
	
	public StubMonitorView()
	{
		
	}

	@Override
	public void setMonitor()
	{
		stubCall = true;
	}

	@Override
	public void close()
	{
		
	}

	@Override
	public < E > void setMonitorState(E v)
	{
		
	}

	@Override
	public void addController(ActionListener listenerButtons)
	{
		stubCall = false;
	}

}

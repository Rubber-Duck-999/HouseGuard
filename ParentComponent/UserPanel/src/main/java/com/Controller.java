package com;

import java.awt.event.ActionListener;
import java.awt.event.ActionEvent;
import com.house_guard.Common.*;
import java.util.logging.Logger;

public class Controller implements ActionListener
{
	public Model _model;
	public View _view;
	private MonitorView _monitorView;
	private ConsumerTopic _consumer;
	private RequestTable _pinTable;

	public Controller(Model m, View v, MonitorView monitorView, ConsumerTopic consumer, RequestTable requestTable)
	{
		this._model = m;
		this._view = v;
		this._monitorView = monitorView;
		this._consumer = consumer;
		_pinTable = requestTable;
	}
	
	public void enterCommand()
	{
		Integer val = _model.checkPass();
		Integer key = _pinTable.addRecordNextKeyReturn(val);
		_consumer.askForAccess(key, val);
	}
	
	public void checkAccess()
	{
		if(!_consumer.getAccessStateSet())
		{
			_view.displayPassMessage("Loading...");
			if(_consumer.getAccessState() && _pinTable.doesKeyExist(_consumer.getId()))
			{
				_view.displayPassMessage("Pass");
				this._monitorView.setMonitor();
			}
			else
			{
				_view.displayErrorMessage("Wrong Passcode");
			}
		}
	}

	public void actionPerformed(java.awt.event.ActionEvent e)
	{
		String input = e.getActionCommand();
		checkAction(input);
	}

	public void checkAction(String input) 
	{
		if(Types.Actions.ENTER.name().equals(input))
		{
			this.enterCommand();
			this.checkAccess();
			_view.setDigits(_model.initModel(Types.RESET));	
		}
		else if(Types.Actions.ADD_D1.name().equals(input))
		{
			_view.setDigit1(_model.incrementValue(Types.D1));
		}
		else if(Types.Actions.SUB_D1.name().equals(input))
		{
			_view.setDigit1(_model.decrementValue(Types.D1));
		}
		else if(Types.Actions.ADD_D2.name().equals(input))
		{
			_view.setDigit2(_model.incrementValue(Types.D2));
		}
		else if(Types.Actions.SUB_D2.name().equals(input))
		{
			_view.setDigit2(_model.decrementValue(Types.D2));
		}
		else if(Types.Actions.ADD_D3.name().equals(input))
		{
			_view.setDigit3(_model.incrementValue(Types.D3));
		}
		else if(Types.Actions.SUB_D3.name().equals(input))
		{
			_view.setDigit3(_model.decrementValue(Types.D3));
		}
		else if(Types.Actions.ADD_D4.name().equals(input))
		{
			_view.setDigit4(_model.incrementValue(Types.D4));
		}
		else if(Types.Actions.SUB_D4.name().equals(input))
		{
			_view.setDigit4(_model.decrementValue(Types.D4));
		}
		else if(Types.State.OFF.name().equals(input))
		{
			_monitorView.setMonitorState(_model.setModelStateOFF());
			this.sendMonitorUpdate(false);
			_monitorView.close();
		}
		else if(Types.State.ON.name().equals(input))
		{
			_monitorView.setMonitorState(_model.setModelStateOn());
			this.sendMonitorUpdate(true);
			_monitorView.close();
		}
	}
	
	public void sendMonitorUpdate(boolean state)
	{
		_consumer.sendMonitorState(state);
	}

	public void initmodel(int x, String state)
	{
		_view.setDigits(_model.initModel(x));
		_monitorView.setMonitorState(state);
	}
}

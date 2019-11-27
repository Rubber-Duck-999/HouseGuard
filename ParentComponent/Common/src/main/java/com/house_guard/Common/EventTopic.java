package com.house_guard.Common;

public class EventTopic 
{
	public String getComponent() 
	{
		return component;
	}
	public void setComponent(String component) 
	{
		this.component = component;
	}
	public String getError() 
	{
		return error;
	}
	public void setError(String error) 
	{
		this.error = error;
	}
	public String getTime() 
	{
		return time;
	}
	public void setTime(String time) 
	{
		this.time = time;
	}
	public Integer getSeverity() 
	{
		return severity;
	}
	public void setSeverity(Integer severity) 
	{
		this.severity = severity;
	}
	private String component;
	private String error;
	private String time;
	private Integer severity;

}

using System;
using System.Linq;
using System.Reflection;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public class Phase146NullableAnnotationTests
{
    [Fact]
    public void AgentSpec_Model_Is_NonNullable_String()
    {
        var prop = typeof(AgentSpec).GetProperties().First(p => p.Name == "Model");
        Assert.Equal(typeof(string), prop.PropertyType);
    }

    [Fact]
    public void AgentSpec_Instructions_Is_String()
    {
        var prop = typeof(AgentSpec).GetProperties().First(p => p.Name == "Instructions");
        Assert.Equal(typeof(string), prop.PropertyType);
    }

    [Fact]
    public void AgentSpec_Tools_Is_Nullable_List()
    {
        var prop = typeof(AgentSpec).GetProperties().First(p => p.Name == "Tools");
        Assert.True(prop.PropertyType.IsGenericType ||
                    Nullable.GetUnderlyingType(prop.PropertyType) != null ||
                    !prop.PropertyType.IsValueType);
    }

    [Fact]
    public void AgentSpec_MaxTokens_Is_Nullable_Int()
    {
        var prop = typeof(AgentSpec).GetProperties().First(p => p.Name == "MaxTokens");
        Assert.Equal(typeof(int?), prop.PropertyType);
    }

    [Fact]
    public void AgentSpec_Temperature_Is_Nullable_Double()
    {
        var prop = typeof(AgentSpec).GetProperties().First(p => p.Name == "Temperature");
        Assert.Equal(typeof(double?), prop.PropertyType);
    }

    [Fact]
    public void ToolSpec_InputSchema_Is_Nullable()
    {
        var prop = typeof(ToolSpec).GetProperties().First(p => p.Name == "InputSchema");
        Assert.True(!prop.PropertyType.IsValueType);
    }

    [Fact]
    public void ToolInputProperty_Description_Is_Nullable_String()
    {
        var prop = typeof(ToolInputProperty).GetProperties().First(p => p.Name == "Description");
        Assert.Equal(typeof(string), prop.PropertyType);
    }

    [Fact]
    public void GraphNode_AgentSpec_Is_Nullable()
    {
        var prop = typeof(GraphNode).GetProperties().First(p => p.Name == "AgentSpec");
        Assert.True(!prop.PropertyType.IsValueType);
    }

    [Fact]
    public void GraphEdge_Condition_Is_Nullable_String()
    {
        var prop = typeof(GraphEdge).GetProperties().First(p => p.Name == "Condition");
        Assert.Equal(typeof(string), prop.PropertyType);
    }

    [Fact]
    public void RunHandle_RunId_Is_NonNull_String()
    {
        var prop = typeof(RunHandle).GetProperty("RunId");
        Assert.Equal(typeof(string), prop!.PropertyType);
    }

    [Fact]
    public void ToolAttribute_Name_Is_Nullable_String()
    {
        var prop = typeof(ToolAttribute).GetProperty("Name");
        Assert.Equal(typeof(string), prop!.PropertyType);
    }

    [Fact]
    public void ToolInputAttribute_Description_Is_Nullable_String()
    {
        var prop = typeof(ToolInputAttribute).GetProperty("Description");
        Assert.Equal(typeof(string), prop!.PropertyType);
    }
}

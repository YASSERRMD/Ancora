using System.Text;
using Ancora;
using Xunit;

namespace Ancora.Tests;

public static class ProviderConstants146
{
    public const string ClaudeOpus = "claude-opus-4-8";
    public const string Gpt4o = "gpt-4o";
    public const string Gemini25Pro = "gemini-2-5-pro";
    public const string MistralLarge = "mistral-large-latest";
    public const string DeepSeekChat = "deepseek-chat";
    public const string QwenMax = "qwen3-max";
    public const string GlmTurbo = "glm-5";
    public const string KimiK2 = "kimi-k2";
}

public class Phase146ProviderSelectionTests
{
    [Fact]
    public void ClaudeOpus_Model_Id_Is_Correct()
    {
        Assert.Equal("claude-opus-4-8", ProviderConstants146.ClaudeOpus);
    }

    [Fact]
    public void Gpt4o_Model_Id_Is_Correct()
    {
        Assert.Equal("gpt-4o", ProviderConstants146.Gpt4o);
    }

    [Fact]
    public void Gemini_Model_Id_Is_Correct()
    {
        Assert.Equal("gemini-2-5-pro", ProviderConstants146.Gemini25Pro);
    }

    [Fact]
    public void Mistral_Model_Id_Is_Correct()
    {
        Assert.Equal("mistral-large-latest", ProviderConstants146.MistralLarge);
    }

    [Fact]
    public void DeepSeek_Model_Id_Is_Correct()
    {
        Assert.Equal("deepseek-chat", ProviderConstants146.DeepSeekChat);
    }

    [Fact]
    public void Qwen_Model_Id_Is_Correct()
    {
        Assert.Equal("qwen3-max", ProviderConstants146.QwenMax);
    }

    [Fact]
    public void GLM_Model_Id_Is_Correct()
    {
        Assert.Equal("glm-5", ProviderConstants146.GlmTurbo);
    }

    [Fact]
    public void Kimi_Model_Id_Is_Correct()
    {
        Assert.Equal("kimi-k2", ProviderConstants146.KimiK2);
    }

    [Fact]
    public void AgentSpec_Accepts_Any_Provider_Model()
    {
        var providers = new[]
        {
            ProviderConstants146.ClaudeOpus,
            ProviderConstants146.Gpt4o,
            ProviderConstants146.Gemini25Pro,
            ProviderConstants146.MistralLarge,
            ProviderConstants146.DeepSeekChat,
        };
        foreach (var p in providers)
        {
            var spec = new AgentSpec(p);
            Assert.Equal(p, spec.Model);
        }
    }

    [Fact]
    public void AgentSpec_With_Provider_Encodes_Model_Id()
    {
        var spec = new AgentSpec(ProviderConstants146.DeepSeekChat);
        var json = System.Text.Encoding.UTF8.GetString(Wire.EncodeAgentSpec(spec));
        Assert.Contains("deepseek-chat", json);
    }
}

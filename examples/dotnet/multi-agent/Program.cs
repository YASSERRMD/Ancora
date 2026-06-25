using System;
using System.Linq;
using System.Text.Json;
using System.Threading.Tasks;
using Ancora;

Console.WriteLine("Ancora .NET SDK: multi-agent verifier example");
Console.WriteLine("==============================================");
Console.WriteLine();

// The drafter generates a short answer.
var drafterSpec = new AgentSpec(
    Model: "llama3",
    Instructions: "Draft a one-sentence answer to the user's question.",
    MaxTokens: 128
);

// The verifier reviews the draft and returns APPROVED or REVISE with a reason.
var verifierSpec = new AgentSpec(
    Model: "llama3",
    Instructions: """
        You are a strict factual verifier.
        Read the provided draft answer and reply with exactly one of:
          APPROVED
          REVISE: <reason>
        """,
    MaxTokens: 64
);

// Shared runtime so both agents talk to the same context.
using var rt = new Runtime();
using var drafter = new Agent(rt);
using var verifier = new Agent(rt);

string question = "What is the capital of France?";
Console.WriteLine($"Question: {question}");
Console.WriteLine();

// --- Step 1: drafter produces an answer ---
var drafterHandle = drafter.Run(new AgentSpec(
    drafterSpec.Model,
    $"{drafterSpec.Instructions}\n\nQuestion: {question}",
    MaxTokens: drafterSpec.MaxTokens));

Console.WriteLine($"Drafter run: {drafterHandle.RunId}");
var drafterEvents = await drafterHandle.CollectAsync();
var draft = string.Concat(
    drafterEvents.OfType<TokenEvent>().Select(t => t.Text));
Console.WriteLine($"Draft answer: {draft.Trim()}");
Console.WriteLine();

// --- Step 2: verifier reviews the draft ---
var verifierHandle = verifier.Run(new AgentSpec(
    verifierSpec.Model,
    $"{verifierSpec.Instructions}\n\nDraft: {draft}",
    MaxTokens: verifierSpec.MaxTokens));

Console.WriteLine($"Verifier run: {verifierHandle.RunId}");
var verifierEvents = await verifierHandle.CollectAsync();
var verdict = string.Concat(
    verifierEvents.OfType<TokenEvent>().Select(t => t.Text)).Trim();
Console.WriteLine($"Verdict: {verdict}");
Console.WriteLine();

if (verdict.StartsWith("APPROVED", StringComparison.OrdinalIgnoreCase))
{
    Console.WriteLine("Final answer accepted.");
    Console.WriteLine($"Answer: {draft.Trim()}");
}
else
{
    Console.WriteLine("Draft was flagged for revision.");
    Console.WriteLine($"Reason: {verdict}");
}

Console.WriteLine();
Console.WriteLine("Drafter cost: " + drafterHandle.GetCost());
Console.WriteLine("Verifier cost: " + verifierHandle.GetCost());

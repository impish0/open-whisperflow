# Prompt Templates Guide

This document contains example prompt templates for different use cases. You can use these as-is or customize them to fit your needs.

---

## How Prompt Templates Work

When you speak, Open WhisperFlow:
1. Transcribes your speech (with filler words, grammar mistakes, etc.)
2. Sends it to an LLM with a prompt template
3. Receives cleaned/formatted text
4. Inserts the result at your cursor

**Variables Available:**
- `{text}` - The raw transcription from Whisper
- `{context}` - The active application name (e.g., "Gmail", "Slack", "VSCode")
- `{language}` - Detected language (e.g., "English", "Spanish")

---

## Built-in Templates

### 1. Minimal (Light Touch)

**Best for:** Note-taking, personal documents, preserving natural speech patterns

```
Clean up this voice transcription by:
1. Removing filler words (um, uh, like, you know)
2. Fixing obvious typos
3. Adding basic punctuation
4. DO NOT change the tone or rephrase sentences

Transcription: {text}

Cleaned:
```

**Example:**

Input: `"um so like I think we should probably uh schedule a meeting"`

Output: `"I think we should probably schedule a meeting"`

---

### 2. Balanced (Default)

**Best for:** General use, emails, documents, chat messages

```
You are a text refinement assistant. Your task is to clean up voice transcriptions while preserving the original meaning and intent.

Instructions:
- Remove filler words (um, uh, like, you know, so, kind of)
- Fix grammar and punctuation
- Improve sentence structure slightly
- Keep the same level of formality
- Preserve technical terms and proper nouns exactly
- DO NOT summarize or significantly rewrite

Transcription: {text}

Refined text:
```

**Example:**

Input: `"so I was thinking we could um maybe use react for the frontend and uh node for the backend you know"`

Output: `"I was thinking we could use React for the frontend and Node for the backend."`

---

### 3. Professional (Business/Formal)

**Best for:** Professional emails, business documents, formal communication

```
You are a professional writing assistant. Transform this voice transcription into polished, professional business communication.

Guidelines:
- Remove all filler words and casual language
- Use formal, professional tone
- Improve sentence structure and flow
- Add appropriate business language
- Maintain clarity and conciseness
- Preserve factual content exactly

Transcription: {text}

Professional version:
```

**Example:**

Input: `"hey so like I wanted to check in about that project thing we talked about"`

Output: `"I wanted to follow up regarding the project we discussed."`

---

### 4. Casual (Chat/Messaging)

**Best for:** Slack, Discord, text messages, informal communication

```
Clean up this voice transcription for casual messaging:
- Remove filler words (um, uh, like)
- Keep the casual, friendly tone
- Use contractions (don't, won't, can't)
- Emojis are fine if appropriate
- Keep it conversational and natural

Transcription: {text}

Casual text:
```

**Example:**

Input: `"um hey so like do you wanna maybe grab coffee tomorrow"`

Output: `"hey, wanna grab coffee tomorrow?"`

---

### 5. Technical (Code Comments & Documentation)

**Best for:** IDE/code editors, technical writing, documentation

```
Clean up this voice transcription for technical content:
- Remove filler words
- Use precise technical language
- Maintain technical accuracy
- Format appropriately for code comments or documentation
- Preserve function names, variables, and technical terms exactly

Context: {context}
Transcription: {text}

Technical text:
```

**Example:**

Input: `"so this function um basically takes an array and like sorts it using quicksort"`

Output: `"This function takes an array and sorts it using quicksort."`

---

### 6. Email Compose

**Best for:** Gmail, Outlook, email composition

```
Transform this voice transcription into a well-formatted email.

Guidelines:
- Remove filler words
- Structure as proper email with greeting if mentioned
- Use appropriate email tone (professional but friendly)
- Add paragraph breaks for readability
- Preserve recipient names and specific details

Transcription: {text}

Email:
```

**Example:**

Input: `"hi john um I wanted to follow up on our meeting yesterday about the uh marketing campaign and see if you have any updates"`

Output:
```
Hi John,

I wanted to follow up on our meeting yesterday about the marketing campaign and see if you have any updates.
```

---

### 7. Creative Enhancement

**Best for:** Creative writing, storytelling, expressive content

```
Enhance this voice transcription to be more vivid and engaging while preserving the core message:
- Remove filler words
- Add descriptive language where appropriate
- Improve flow and rhythm
- Keep the original emotion and intent
- Make it more engaging to read

Transcription: {text}

Enhanced version:
```

**Example:**

Input: `"the sunset was really pretty with like orange and pink colors"`

Output: `"The sunset was stunning, painting the sky in vibrant shades of orange and pink."`

---

### 8. Concise (TL;DR Mode)

**Best for:** Long-winded dictations that need to be condensed

```
Condense this voice transcription to its essential points:
- Remove all filler words
- Eliminate redundancy
- Keep only key information
- Make it brief and clear
- Preserve critical details

Transcription: {text}

Concise version:
```

**Example:**

Input: `"so um I think what I'm trying to say is that we need to like finish this by Friday because uh the deadline is coming up and we can't miss it"`

Output: `"We need to finish this by Friday to meet the deadline."`

---

### 9. Expand & Clarify

**Best for:** Quick notes that need to be fleshed out

```
Expand this brief voice transcription into clear, complete sentences:
- Remove filler words
- Add context and clarity
- Make implicit information explicit
- Use complete sentences
- Maintain the original meaning

Transcription: {text}

Expanded version:
```

**Example:**

Input: `"meeting 3pm john sarah budget review"`

Output: `"Schedule a meeting at 3pm with John and Sarah to review the budget."`

---

### 10. Context-Aware (Auto-Detect)

**Best for:** Advanced use - adapts to the application you're using

```
Clean up this voice transcription with the appropriate tone for the context.

Application context: {context}
Transcription: {text}

Instructions:
- If in email app: Use professional email tone
- If in chat app (Slack, Discord): Use casual, friendly tone
- If in IDE/code editor: Use technical, precise language
- If in document editor: Use balanced, formal tone
- Always remove filler words and improve grammar

Formatted text:
```

---

## Advanced: Custom Variables

You can create custom templates with conditional logic:

```
Application: {context}
Language: {language}

Clean up this transcription:
{text}

Rules:
- Remove filler words in {language}
- Adapt tone for {context}
- Use appropriate formality level
```

---

## Tips for Creating Your Own Templates

### 1. Be Specific About What NOT to Do

```
DO NOT:
- Summarize the content
- Change technical terms
- Add information not in the original
- Change the level of formality dramatically
```

### 2. Provide Examples in the Prompt (Few-Shot)

```
Example transformations:

Input: "um so like I need to uh send an email"
Output: "I need to send an email"

Input: "we should probably maybe consider doing that"
Output: "We should consider doing that"

Now clean up this text:
{text}
```

### 3. Use Chain-of-Thought for Complex Tasks

```
Step 1: Identify and remove filler words
Step 2: Fix grammar and punctuation
Step 3: Adjust tone if needed
Step 4: Review for clarity

Transcription: {text}

Final result:
```

### 4. Set Temperature/Parameters (Advanced)

Some templates work better with specific LLM settings:

- **Low Temperature (0.3)**: Predictable, conservative changes (Professional, Technical)
- **Medium Temperature (0.7)**: Balanced creativity and accuracy (Default)
- **High Temperature (1.0)**: More creative, expressive (Creative Enhancement)

---

## Testing Your Templates

### A/B Testing

Open WhisperFlow allows you to test multiple templates on the same audio:

1. Record your audio
2. In the result window, click "Try Another Template"
3. Compare outputs side-by-side
4. Save the best one

### Template Metrics

Track which templates work best:
- Success rate (how often you keep the output vs. manual edit)
- Average edit distance (how much you change the result)
- Speed (how long the LLM takes)

---

## Sharing Templates

### Export Your Template

Settings > Prompt Templates > [Your Template] > Export

Creates a `.json` file:

```json
{
  "name": "My Custom Template",
  "description": "Perfect for my use case",
  "prompt": "Your prompt text here...",
  "variables": ["text", "context"],
  "recommended_temperature": 0.7,
  "recommended_models": ["gpt-4o-mini", "llama3.2:3b"]
}
```

### Import Community Templates

Settings > Prompt Templates > Import

Browse community templates at: [github.com/openwhisperflow/templates](https://github.com/openwhisperflow/templates)

---

## Template Performance Guide

### Fast Templates (< 1 second with local LLM)

- Minimal
- Casual
- Concise

**Why**: Simple instructions, minimal transformation

### Medium Templates (1-2 seconds)

- Balanced
- Professional
- Technical

**Why**: Moderate complexity, some context awareness

### Slow Templates (2-3 seconds)

- Creative Enhancement
- Context-Aware
- Expand & Clarify

**Why**: Complex transformations, more reasoning required

---

## Language-Specific Templates

### Spanish

```
Limpia esta transcripción de voz:
- Elimina muletillas (este, pues, o sea, como que)
- Corrige gramática
- Mejora la estructura

Transcripción: {text}

Texto limpio:
```

### French

```
Nettoyez cette transcription vocale:
- Supprimez les mots de remplissage (euh, ben, alors, genre)
- Corrigez la grammaire
- Améliorez la structure

Transcription: {text}

Texte nettoyé:
```

### German

```
Bereinigen Sie diese Sprachtranskription:
- Entfernen Sie Füllwörter (ähm, also, sozusagen)
- Korrigieren Sie Grammatik
- Verbessern Sie die Struktur

Transkription: {text}

Bereinigter Text:
```

---

## Troubleshooting Template Issues

### Template is Changing Meaning

**Solution**: Add explicit instructions:

```
CRITICAL: Preserve the exact meaning and all factual content.
Only clean up grammar and remove filler words.
DO NOT add, remove, or change any information.
```

### Template is Too Conservative

**Solution**: Be more explicit about improvements:

```
Instructions:
- Remove filler words (mandatory)
- Fix grammar (mandatory)
- Improve sentence structure (encouraged)
- Enhance clarity (encouraged)
```

### Template is Too Slow

**Solutions**:
1. Simplify the prompt (fewer instructions)
2. Remove examples (few-shot learning adds tokens)
3. Use a faster model (llama3.2:3b vs mistral:7b)
4. Use cloud API (often faster than local)

### Template Results are Inconsistent

**Solution**: Add constraints and lower temperature:

```
Be consistent in your transformations.
Use the same style for similar inputs.

Temperature: 0.3 (set in app settings)
```

---

## Advanced: System Prompts

For even more control, you can set a system-level prompt that applies to ALL templates:

```
You are a voice transcription refinement assistant.
Your goal is to clean up speech-to-text output while preserving intent.
Always be conservative - when in doubt, keep the original.
Never add information that wasn't in the original.
```

Set this in: Settings > Advanced > System Prompt

---

## Contributing Templates

Share your templates with the community!

1. Export your template
2. Test it with various inputs
3. Write a clear description and examples
4. Submit to: [github.com/openwhisperflow/templates](https://github.com/openwhisperflow/templates)

**What makes a great community template:**
- Clear, specific use case
- Well-tested with examples
- Documented edge cases
- Works with both cloud and local LLMs

---

Ready to create your perfect prompt? Open the app and go to Settings > Prompt Templates > Create New!

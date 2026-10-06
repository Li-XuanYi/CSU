# GPT4 学术prompt

2024.04.05 by MeBr



## 一、学术角色预设（8个）

论文中无论是润色、语法检查还是降重，都要先进行角色预设，先给定一个身份。

有些学术GPT是自带提示词的，有些人使用的GPT并没有提前进行预设，这种需要我们手动进行预设。

例如你需要问论文方面的问题，可以先用这个指令对GPT提问，提前预设好GPT角色了，会让接下来的提问结果质量更高，语法更具准确性。

### **1.学术角色**

> As a leader in the academic field, I possess extensive academic experience and professional knowledge across various domains. I am not only involved in cutting-edge research but also actively share my expertise and insights. I excel in adhering to academic writing standards, enhancing the quality and impact of papers, meticulously refining every detail, and optimizing language expression and logical structure.
>
> 我作为学术领域的引领者，在各个领域拥有丰富的学术经验与专业知识，不仅参与前沿研究，还积极分享经验与见解擅长学术写作规范，提升论文的品质与影响力，精细润色每个细节，优化语言表达与逻辑结构

当然，你也可以预设其他的角色。

### **2.论文评审专家**

- 请他扮演一个专业的论文评审专家，对论文草稿给出评审意见，然后根据意见，去重新审视论文。
- 在修改具体论文内容时，让他扮演我所研究的领域的专家，这样可以让它的表达更加准确。

> You are now acting as an expert in the field of [Put professional fields here…]. From a professional point of view, do you think there is any need to modify the above content? Be careful not to modify the whole text, you need to point out the places that need to be modified one by one, and give revision opinions and recommended revision content.
>
> 你现在扮演一个[这里放你所研究的领域] 领域的专家，从专业的角度，您认为上面这些内容是否有需要修改的地方？ 注意，不要全文修改，您需要一一指出需要修改的地方，并且给出修改意见以及推荐的修改内容



### 3.**论文逻辑性和连贯性评审**

> You are an expert reviewer for a top-tier journal. Please analyze the logical relationships and coherence within the paragraphs in the following text. ldentify areas where fluency or paragraph transitions can be improved, and provide specific suggestionsto enhance the overall quality and readability. Please alscprovide suggestions for modifying sentences. However, do noprovide the original text.
>
> 你是一个**顶级期刊的评审专家**，请**分析以下文本中各段落的逻辑关系和连贯性。**请指出那些流畅性或段落间联系可以被改善的地方，并提供具体建议，以提升整体的质量和可读性。**请同时也对句子提供修改建议。但是，不要提供原始文本。**



### 4.**评价论文结构评审**

> You are an expert reviewer for a top-tier journal. Please analyze whether the organizational structure of this paper is clear. Determine if the introduction, methods, results, discussion and conclusion are logically coherent, enabling readers to understand the author's research purpose, implementation methods, major findings, and the implications of these findings. Please provide critical feedback.
>
> 你是一个**顶级期刊的评审专家**，请**分析这篇论文的组织结构是否清晰。**请确认引言、方法、结果、讨论和结论是否在逻辑上连贯，使读者能明白作者的研究目的、实施方式、主要发现以及这些发现的含义。**请提出批判性的意见。**



### 5.**研究科学性评审**

> You are an expert reviewer for a top-tler journal, Please analyze whether the research design of the following paper is scientifically sound, Determine if the methods and experimental design employed are appropriate for the research question addressed in the paper. Assess whether the sample size used is adequate, Please provide critical feedback.
>
> 你是一个**顶级期刊的评审专家**，请**分析下面这篇论文的研究设计是否科学合理。**请确认使用的方法和实验设计是否适合论文所研究的问题。**请评估使用的样本量是否合适。请提出批判性的意见。**



### 6.**结果推论一致性评审**

> You are an expert reviewer for a top-tier journal. Please analyze whether the discussion in this article aligns with the results. Assess if the content in the discussion section is based on the results and if it extends beyond what the results can support. Determine if the discussion section contains numerous unsubstantiated assumptions or speculations, ldentify the viewpoints within the discussion that lack sufficient evidence, Evaluate whether the discussion provides a comprehensive explanation of the results. Assess if it connects with existing research, compares the author .findings with other studies, and explain potential similarities and differences. Please provide critical feedback.
>
> 你是一个**顶级期刊的评审专家**，这**分析这篇文章的讨论是否与结果一致?**讨论部分的内容是否基于结果部分，是否超出结果所能支持的范围?讨论部分是否包含了大量未经证实的假设或推测?讨论中有哪些观点是没有足够论据支持的?该讨论是否深入地解释了结果?是否与现有的研究相联系?请比较这文章的结果与其他研究的结果，解释可能的异同。**请提出批判性的意见。**



### 7.**数据实验严谨性评审**

> Please compare the data reported in the results section withthe data in Tables X, and correct any discrepancies found. Please provide critical feedback.
>
> 请把结果部分报告的**数据和表格**中的数据进行对比**，**并将结果部分的**偏差进行矫正。**请提出批判性的意见。



### 8.**整篇论文发表角度评审**

> You are an expert reviewer for a top-tier journal. Please evaluate this article based on the publication standards and provide as many suggestions for revisions as possible.
>
> 你是一个**顶级期刊的评审专家**，请从国际顶会、期刊**发表的标准**评判这篇文章，并尽可能多地提出修改意见。



## 二、论文撰写指令（8个）

### **1.写论文标题**

> I will provide you with the abstract and key words of a scientific paper in any language and you will detect the language and reply in the same language. Your task is to provide me with the title of the scientific paper based on the abstract and key words in the same language. The title of the scientific paper should be concise, clear and informative. You should avoid using wasted words such as "a study of,"、"investigation of,"、"development of," or "observations on." Make sure the title can grip the audience immediately. My abstract is "XXX", my key words are "XXX"
>
> 我将向你提供一篇任何语言的科学论文的摘要和关键词，你将检测该语言并以相同的语言进行回复。你的任务是根据摘要和关键词用相同的语言向我提供科学论文的标题。科学论文的标题应该是简洁、明确和有信息量的。你应该避免使用诸如研究、调查、发展或观察等词语。确保标题能够立即抓住听众的心。

### **2.写英文标题**

> I want you to act as an academic journal editor. I am going to provide you an abstract of manuscript and you provide me with 5 good titles in English for a research paper and give explanation for why this title is good. Provide your output as a markdown table with two columns and with head in Chinese. First column gives titles in English and second column provides explanation in Chinese. The following text is the abstract:
>
> 希望你担任一名学术期刊编辑。我将为你提供一个手稿的摘要，你需要为一篇研究论文提供5个好的英文标题，并解释为什么这个标题好。请以Markdown表格的形式提供你的输出，表格有两列，标题用中文。第一列给出英文标题，第二列用中文提供解释。以下是摘要：

### **3.写论文摘要**

这个指令可以为你的研究论文撰写一个摘要，简洁地总结研究目标、方法、主要发现和意义，确保摘要清晰、连贯，同时遵循学术领域的具体指南和惯例。

> Act as an academic research expert. Draft an abstract for a research paper titled [title]. The abstract should succinctly summarize the main objectives, methodologies, key findings, and implications of the research.
>
> 作为学术研究专家，为研究论文撰写一个简洁、精确的摘要。

### **4.写英文摘要**

> Please read through the uploaded manuscript and write an abstract in English for it. The abstract should initiate with a comprehensive summary of the broader context or background of the study, followed by a statement that describe the gaps, limitations or issues. Then, describe the research methods used in the manuscript. After that, write 3-5 sentences showing the key findings. In the end, include a statement which underscores the unique value or significant contribution of the manuscript. After generating the abstract, give explanation in Chinese checking if you have followed the instruction in a markdown table.
>
> 请阅读上传的手稿，并为其撰写一份英文摘要。摘要应首先提供研究背景或更广泛背景的全面概述，接着陈述描述研究的空白、局限性或问题。然后，描述手稿中使用的研究方法。此后，写出3-5句话展示关键发现。最后，包括一份强调手稿独特价值或重大贡献的声明。在生成摘要后，以中文提供解释，检查你是否遵循了指令，并以Markdown表格形式展示。

### **5.缩写名称**

可以向AI寻求为段落起标题，为方法起缩写名称等。

> What abbreviations can "XXX" have? Give several options, with reasons, for use in an academic paper. "XXX"
>
> 可以有哪些缩写？请给出几种选择，并给出理由，以便用于论文中。

### **6.论文续写**

这个方法一般适合实在想不出什么内容，又希望增加字数的情况。

> Based on the knowledge you have mastered about [xxx], polish and continue writing the above content to make the content richer and more complete.
>
> 根据你所掌握的关于[xxx]的知识，润色并续写上面的内容，使得内容更加丰富完整。

### **7.论文致谢**

对在研究过程中提供帮助的个人或机构表示感谢。

> 我想请你帮我写一份关于我的论文的致谢。我的论文的题目是(题目)，我的导师是(导师)，我的合作者是(合作者)。我想感谢以下的人或机构：
> (感谢对象1):感谢他们对我的(帮助或贡献)
> (感谢对象2):感谢他们对我的(帮助或贡献)
> (感谢对象n):感谢他们对我的(帮助或贡献)
> 你能根据这些信息，写一份大约(字数)字的致谢吗?请使用礼貌和诚恳的语气并注意格式和标点。

### **8.论文大纲**

为研究论文起草一个结构良好的大纲，包括引言、文献综述、方法论、研究结果和结论，确保研究的合理性、创新性和对现有知识的贡献。

> Act as an academic research expert. Draft a comprehensive research paper outline on [topic]. The outline should be well-structured, starting with a compelling introduction that states the problem or question, the relevance of the topic, and the objectives of the research.
>
> 作为学术研究专家，为研究论文起草一个结构良好的大纲，明确研究的主要部分。

## **三、学术润色指令**（20个）

学术润色涉及对论文草稿的语言、语法、逻辑和结构进行改进，以提高其清晰度、连贯性和专业性。

### **1.英文润色**

> The following is a paragraph from a n academic paper. Refinish writing to conform to academic style，improve spelling，grammar，clarity, conciseness and overall readability. If necessary, rewrite the entire sentence. In addition,list all modifications in the Markdown table and explain the reasons for doing so.Paragraph ：（+the paragraph that requires polishing）
>
> 以下是一篇学术论文中的一段文字。请重新润色写作，以符合学术风格，提高拼写、语法、清晰度、简洁性和整体可读性。如有必要，重写整个句子。此外，请在Markdown表格中列出所有修改，并解释修改的原因。段落：（+润色内容）。

### **2.英文润色**

这段指令将在Markdown表格中列出所有润色修改部分，并解释修改的原因，可以看到润色的效果很不错的，也可以优化提问。润色过程中，可以让GPT提供多个版本的修改建议，我们的选择性也更多。

> Below is a paragraph from an academic paper. Polish the writing to meet the academic style,improve the spelling, grammar, clarity, concision and overall readability. When necessary, rewrite the whole sentence. Furthermore, list all modification and explain the reasons to do so in markdown table. Paragraph ：XXX
>
> 以下是一篇学术论文中的段落。请润色写作以符合学术风格，提高拼写、语法、清晰度、简洁性和整体可读性。如有必要，重写整个句子。此外，请在Markdown表格中列出所有修改，并解释修改的原因。段落：XXX

### **3.中文润色**

与通常的指令不同的是，这个prompt特别指出，助手只需提供修改后的文本版本，而无需包含任何解释或额外的说明。

> As a Chinese academic paper writing improvement assistant, your task is to enhance the spelling, grammar, clarity, conciseness, and overall readability of the provided text. Break down long sentences, reduce repetition, and offer suggestions for improvement. Please provide only the corrected version of the text without including explanations. Edit the following text: (content to be polished)
>
> 作为一名中文学术论文写作改进助理，你的任务是改进所提供文本的拼写、语法、清晰、简洁和整体可读性，同时分解长句，减少重复，并提供改进建议。请只提供文本的更正版本，避免包括解释。请编辑以下文本：（润色内容）

### **4.SCI论文润色**

> I am preparing my SCI paper for submission and require assistance in polishing each paragraph. Could you please refine my writing for academic rigor? I need you to correct any grammatical errors, improve sentence structure for academic suitability, and make the text more formal where necessary. For each paragraph we need to improve, you need to put all modified sentences in a Markdown table, each column contains the following: Full original sentence; Highlight the revised part of the sentence; Explain why made these changes. Finally, Rewrite the full, corrected paragraph. If you understand, please reply: yes, let's get started.
>
> 我正在准备我的SCI论文以便提交，需要帮助润色每个段落。你能帮我提升学术严谨性吗？我需要你纠正任何语法错误，改进句子结构以适应学术要求，并在必要时使文本更加正式。对于每个需要改进的段落，你需要将所有修改后的句子放在一个Markdown表格中，每一列分别包含以下内容：完整的原始句子；突出显示句子的修订部分；解释为什么做出这些更改。最后，重写整个更正后的段落。如果你理解了，请回复：是的，让我们开始吧。

### **5.期刊/会议风格**

根据期刊会议(注意 期刊或者会议要足够著名)的风格，来润色内容。

> If I wish to publish a paper at a XXX conference, please polish the above content in the style of a XXX article.
>
> 提示：如果我希望将论文发表在XXX会议/期刊上，请按照XXX文章的风格，对上面的内容进行润色。

### **6.润色英文段落结构和句子逻辑**

> I am a researcher studying +（你的研究方向） and now trying to revise my manuscript which willbe subrnitted to the +（你的投稿期刊）. want you to analyze the logic and coherence amongsentences within each paragraph in the following text, ldentify any areas where the flow orconnections between sentences could be improved,and provide specific suagestions toenhance the overall quality and readabllity to the content. Please only provide the text aftelimproving and then give a list of the improvements in Chinese. lf you understand the abovetask, please reply with yes, and then I will provide you with the text.

### **7.直接润色段落**

> Polish the paragraph above to make it more logical, and academic.
>
> 润色上面的内容，使其更加更合逻辑，更符合学术风格。

### **8.润色多版参考**

> Please provide multiple versions for reference
>
> 这里给出了其它两版的参考，方便我们参考对比

### **9.错误纠正**

如果ChatGPT理解错了你的问题，可以给它一个错误的反馈，让它重新回答

> Prompt: Note that it is not ....., but ..... Re-answer the previous question based on what I have added. 注意，
>
> 不是...而是... 请根据我的补充，重新回答上个问题

### **10.重新回答**

如果认为回答的不够好，或者方向不对。可以要求重新回答，并且指明侧重方向。比如你只希望去除当前段落的冗余，并不想改动原意思。

> Still the above question, I think your answer is not good enough. Please answer again, this time focusing on removing redundancy from this passage.
>
> 还是上面的问题，我认为你回答的不够好。请重新回答一次，这次你应该侧重于去除这段话中的冗余。

### **11.语法检查/查找语法错误**

> Can you help me ensure that the grammar and the spelling is correct? Do not try to polish the text, if no mistake is found, tell me that this paragraph is good. If you find grammar or spelling mistakes, please list mistakes you find in a two-column markdown table, put the original text the first column, put the corrected text in the second column and highlight the key words you fixed. Example: Paragraph: How is you? Do you knows what is it? | Original sentence | Corrected sentence | | :--- | :--- | | How is you? | How are you? | | Do you knows what is it? | Do you know what it is? | Below is a paragraph from an academic paper. You need to report all grammar and spelling mistakes as the example before. Paragraph: XXX
>
> 你能帮助我确保语法和拼写正确无误吗？不要尝试润色文本，如果没有发现错误，请告诉我这段话很好。如果你发现了语法或拼写错误，请按照之前的例子，在双列的Markdown表格中列出你发现的错误，第一列放原始文本，第二列放更正后的文本，并突出显示你修正的关键词。示例： | 原始句子 | 更正后的句子 | | :--- | :--- | | How **is** you? | How **are** you? | | Do you **knows** what **is it**? | Do you **know** what **it is**? |
> 以下是一篇学术论文中的段落。你需要按照上述例子报告所有语法和拼写错误。段落：XXX

### **12.语法校正**

> I am a researcher studying +（你的研究方向） and now trying to revise my manuscript which willbe submitted to the +（你的投稿期刊）. Please help me to ensure the grammar and spellingare correct. Do not try to improve the text, if no mistake found, tell me this paragraph is good.If you find grammar or spelling mistakes, please list the mistakes you find in a two-columnmark down table, put the original text in the first column, put the corrected text in the second column, and do highlight the key words you fixed in bold.

### **13.语法句法**

> This sentence is grammatically incorrect. Please revise.
> 这句话在语法上是不正确的。请修改。
> The subject and verb do not agree in this sentence. Please correct.
> 主语和动词在这句话中不一致。请改正。
> This phrase seems out of place. Please rephrase to improve clarity.
> 这句话似乎不合适。请重新措辞以表达更清晰。
> I have used a passive voice in this sentence. Consider using an active voice instead.
> 我在这句话中使用了被动语态。考虑改用主动语态。

### **14.润色定位**

如果文本过长不利于观察，让它回答具体修改了哪些地方。

> Note that in addition to giving the modified content, please also indicate which paragraphs and sentences have been modified in the revised version.
> 注意，除了给出润色修改之后的内容，还请指明修订的版本中具体修改了哪些段落的哪几句话。

### **15.优化语法**

> This sentence is grammatically incorrect. Please revise.
> 请修改这句话在语法上不正确的地方
> The subject and verb do not agree in this sentence. Please correct.
> 请改正主语和动词在这句话中不一致的地方
> This phrase seems out of place. Please rephrase to improve clarity.
> 这句话似乎不合适。请重新措辞以表达更清晰。
> I have used a passive voice in this sentence. Consider using an active voice instead.
> 我在这句话中使用了被动语态。考虑改用主动语态。

### **16.修改建议**

> You are now acting as an expert in the field of lung cancer From a professional point of view, do you think there is any need to modify the above content? Be careful not to modify the whole text, you need to point out the places that need to be modified one by one, and give revision opinions and recommended revision content.
> 你现在扮演一个[这里放你所研究的领域] 领域的专家，从专业的角度，您认为上面这些内容是否有需要修改的地方？ 注意，不要全文修改，您需要一一指出需要修改的地方，并且给出修改意见以及推荐的修改内容。

### **17.修改意见**

> I started to write an academic paper, the title is XXXXX, now I have finished the introduction part, but I am not sure whether it is suitable, can you help me to read it, and put forward detailed and specific revision suggestions?
> 我开始写论文了，题目是XXXXX，现在我完成了引言部分，但是不确定是否合适，你能帮我看一下，并提出详细具体的修改意见吗？

### **18.封装基本事实/原理/背景**

润色的同时，修改基本逻辑错误。如果对内容的润色需要一些背景知识，可以在对话时主动告诉ChatGPT，比如XXX原理。

> Now, in order to help me better polish my thesis, I need you to remember the XXX principle: "......."
> 现在，为了接下来能够帮我更好的润色论文，我需要你记住XXX原理：“.......”

这样就相当于为一段内容，封装了一个函数名称，之后你再次提到XXX原理的时候，ChatGPT就能快速知道你说的是哪些基本事实了。

> Polish and rewrite the above content to make it more in line with the style of academic papers, and at the same time, it can be more professional. If there are parts that do not conform to facts or logic, please refer to the part of xxxxx for the above content modification.
> 润色并重写上面的内容，使其更加符合论文的风格，于此同时，又能更加专业化，如果有不符合事实或者逻辑的部分，请你参考XXX原理部分对上面的内容修改。

### **19.逻辑论证辅助**

AI在逻辑推理方面有显著的提升，可以用于辅助构建更有说服力的论证。

> Please help me analyze and optimize the logical structure of this argument to make it more convincing.
> 请帮我分析和优化这段论证的逻辑结构，以使其更具说服力。

### **20.个性化润色指令**

> 更精确的术语(More precise)：选择更精确的词汇，例如使用“generate”代替“produce”
> 精炼表达(More concise)：去除冗余的表达以提高句子的清晰度和直接性。
> 客观的语言(More objective)：剔除含糊和主观性表述，以客观方式呈现信息。
> 细化描述(More specific)：提供更具体的细节，以支持论点或想法。
> 更连贯的表达(More coherent)：确保句子的组织性良好，逻辑流畅。
> 保持风格一致(More consistent)：确保用词和语调与整篇论文保持一致。
> 符合学术风格(More academic)：运用正确的学术用语如“moreover”和“consequently”。
> 规范语法(More formal grammar)：使用正确的语法或句法，避免语句不完整或偏离主题。
> 深化细节描绘(More nuanced)：使用精准的词汇和短语描述复杂或微妙的概念，使句子更加细化。
> Make nuanced adjustments:对文本进行微调
> lmplement marginal modifications:进行边际性修改
> Clarify through rewording:改述以增强清晰性
> Streamline sentence composition:简化句子结构
> Verify grammatical correctness and orthography:校验语法的拼写的正确性
> lmprove textual fluidity and consistency:提升文本的流畅度和连贯性
> Refine diction：措辞精练
> Adjust for stylistic alignment：调整风格
> Execute substantial revisions：执行重要的编辑
> Overhaul content framework:改变内容架构

## 四、中英翻译指令（5个）

### **1.论文翻译**

> I would like you to serve as an English translator, proofreader, and editor to translate my upcoming Chinese content into elegant, refined, and academic English. Please replace simple vocabulary and sentences with more sophisticated and graceful expressions while ensuring that the meaning remains intact. Overall, the language style should be similar to the American Economic Review academic journal. If you understand, please provide an example first.
>
> 我希望您能担任我的英文翻译、校对和编辑工作，将我即将推出的中文内容翻译成优雅、精炼且具有学术性的英文。请在保持原意不变的前提下，将简单的词汇和句子替换为更复杂、更优美的表达方式。总体而言，语言风格应类似于《美国经济评论》学术期刊。如果您理解了，请先提供一个示例。

### **2.中译英**

> Please translate following sentence to English:XXX
> 请将以下句子翻译成中文：XXX

### **3.中译英**

> Translate the above Chinese into the corresponding English, requiring the writing style of an academic paper
> 将上面的中文，翻译成对应的英语，要求具有论文的写作风格

### **4.中英互译①**

> I am a researcher studying **+（**Your research direction**）** and now trying to revise my manuscript which willbe submitted to the**+**（Your submission journal）. I want you to act as a scentiic English-Chnesetranslator,I will provide you with some paragraphs in one language and your task is toaccurately and academically translate the paragraphs only into the other language. I want you to give the output in a markdown table where the first colurrn is the onginal language andthe second is the first version of translation and third column is the second version of thetranslation, and give each row only one sentence. lf you understand the above task, pleasereply with yes, and then l will provide you with the paragraphs.
>
> 我是一名研究者，专注于+（你的研究方向），目前正在修订我的手稿，准备提交至+（你的投稿期刊）。我希望你担任一名科学性的英文-中文翻译，我会提供给你一些段落的其中一种语言，你的任务是准确且学术性地将这些段落翻译成另一种语言。我希望你以Markdown表格的形式给出翻译结果，其中第一列是原文，第二列是第一版的翻译，第三列是第二版的翻译，并且每行只包含一句翻译。如果你理解了上述任务，请回复“是的”，然后我会提供给你这些段落。

### **5.中英互译②**

> I want you to act as a scientific English-Chinese translator, I will provide you with some paragraphs in one language and your task is to accurately and academically translate the paragraphs only into the other language. Do not repeat the original provided paragraphs after translation. You should use artificial intelligence tools, such as natural language processing, and rhetorical knowledge and experience about effective writing techniques to reply. I'll give you my paragraphs as follows, tell me what language it is written in, and then translate:XXX
>
> 我希望你担任一名科学性的英文-中文翻译员，我会提供一些段落给你，你的任务是准确且学术性地将这些段落翻译成另一种语言。翻译后请不要重复原文段落。你应该使用人工智能工具，比如自然语言处理，以及关于有效写作技巧的修辞知识和经验来进行回复。我将如下提供我的段落，告诉我它是用什么语言写的，然后进行翻译：XXX

## 五、论文查重指令（1个）

用的时候一定不用忘记加入自己的研究领域，如我想让你充当一位计算机视觉领域的专家。

> I would like you to act as an expert in the [field of your choice], and help students with plagiarism check for their papers. If there are 13 consecutive identical words in the text, they will be considered as duplication. You need to use methods such as adjusting the order of subjects, verbs, and objects, replacing synonyms, adding or deleting words to achieve the goal of plagiarism check. Please modify the following paragraph:
>
> 我想让你充当一位[你希望的某个]领域的专家，帮助学生进行论文的去重修改。如果文章中连续13个字一样，就算重复。你需要通过调整主谓宾语序、替换同义词、增减字数等方法，来达到论文去重的目的。请你修改下面这段文字：

## **六、参考文献指令**（2个）

### **1.检查参考文献格式**

> I'd like you to serve as a reference editor for a research manuscript. I will supply you with five reference templates that you should use as guidelines. Following that, I will provide additional references for which you'll need to examine formatting aspects such as punctuation placement and spacing. It is essential that the provided references align cohesively with the five initial templates. Provide me with any necessary corrections or suggestions for improve the text. Give a markdown table with three columns where the first is the original text, second is the fixed text, explanation in the third column, and then provide all fixed references. Below are the five example templates and references needed to be fixed:
>
> 我希望您能担任一篇研究手稿的参考文献编辑。我将为您提供五个参考文献模板，您应该将其作为指导方针使用。之后，我将提供额外的参考文献，您需要检查诸如标点符号位置和间距等格式方面的问题。所提供的参考文献必须与最初的五个模板保持一致性。请向我提供任何必要的更正建议或改进文本的建议。请提供一个Markdown表格，表格有三列，第一列是原文，第二列是更正后的文本，第三列是解释，然后提供所有已更正的参考文献。以下是五个示例模板和需要更正的参考文献：

### **2.按照** **APA 格式校正参考文献格式**

> Please first correct the following reference format according to APA style, adjusting it to strictly comply with APA citation format. Finally, I need the references to be displayed in a Markdown format code block. It is important to note that the journal names should be in full and italicized (additional requirements can be added here). Here are my references:
>
> 首先请按照 APA 格式对以下参考文献格式进行校正，调整为严格符合 APA 的文献格式，最后我需要将参考文献以 Markdown 格式的代码块形式展示。需要注意的是期刊名称要全称，且斜体(这里可以添加其他要求)，下面是我的参考文献：

## 七、投稿审稿指令（2个）

### **1.撰写 Cover letter**

> I want you to act as an academic journal editor. I will provide you with the title and abstract of my manuscript. You need to write a format cover letter for submitting the manuscript to the Nature journal. You should state that the manuscript did not consider for publication in any other journal. Briefly introduce the merit of the manuscript and provide a short summary to point out the importance of the results for a scientific audience. The title and abstract are as follows:
>
> 我希望您能担任一名学术期刊编辑。我将为您提供我的手稿的标题和摘要。您需要为将手稿提交给《自然》杂志撰写一封格式正确的封面信。您应该声明该手稿尚未在任何其他期刊上考虑发表。简要介绍手稿的优点，并提供一个简短的总结，以向科学界突出研究结果的重要性。标题和摘要如下： [XXX]

### **2.解释审稿人反馈**

这个指令可以帮你分析和解释审稿人对提交的研究论文的反馈，识别关键问题和建议。然后创建一个详细的回应计划，说明如何在修订稿中解决或反驳每个点。

> Act as an academic research expert. Carefully analyze and interpret the [feedback] provided by the reviewer on the submitted research paper. Identify key concerns, constructive suggestions, and areas of improvement highlighted by the reviewer.
> 作为学术研究专家，分析审稿人的反馈并创建详细的回应计划。

## 八、其他场景的学术指令（11个）

### **1.论文期刊匹配**

> I want you to act as a scientific manuscript matcher. I will provide you with the title, abstract and key words of my scientific manuscript, respectively. Your task is analyzing my title, abstract and key words synthetically to find the most related, reputable journals for potential publication of my research based on an analysis of tens of millions of citation connections in database, such as Web of Science, Pubmed, Scopus, ScienceDirect and so on. You only need to provide me with the 15 most suitable journals. Your reply should include the name of journal, the cooresponding match score (The full score is ten). I want you to reply in text-based excel sheet and sort by matching scores in reverse order.My title is "XXX" My abstract is "XXX" My key words are "XXX"
> 我希望你能充当科学手稿的匹配者。我将分别向您提供我的科学手稿的标题、摘要和关键词。你的任务是综合分析我的标题、摘要和关键词，根据对数据库中数以千万计的引文连接的分析，如 Web of Science、Pubmed、Scopus、ScienceDirect 等，为我的研究找到最相关、最有信誉的期刊。你只需向我提供 15 种最合适的期刊。你的回复应该包括期刊名称，对应的匹配分数（满分是 10 分）。我希望你在基于文本的 excel 表格中进行回复，并按匹配分数倒序排序。

### **2.找图片**

> 我需要你找一张网络图片。使用Unsplash API([https://source.unsplash.com/960x640/?](https://link.zhihu.com/?target=https%3A//source.unsplash.com/960x640/%3F)<英语关键词>)获取图片URL，然后请使用Markdown格式封装，并且不要有反斜线，不要用代码块。现在，请按以下描述给我发送图片：XXX

### **3.解释代码**

> 请解释以下代码：import torch import torch.nn as nn # 定义输入层、隐藏层和输出层的神经元数量 input_layer_size = 4 hidden_layer_size = 5 output_layer_size = 3 # 定义前馈神经网络类 class FeedForwardNN(nn.Module): def init(self): super(FeedForwardNN, self).init() self.fc1 = nn.Linear(input_layer_size, hidden_layer_size) self.fc2 = nn.Linear(hidden_layer_size, output_layer_size) self.sigmoid = nn.Sigmoid() def forward(self, x): x = self.fc1(x) x = self.sigmoid(x) x = self.fc2(x) x = self.sigmoid(x) return x # 创建神经网络实例 model = FeedForwardNN() # 测试前馈函数 X = torch.randn(1, input_layer_size) y = model(X) print(y)

### **4.提供独特见解**

> Please provide me with some unique insights that I can discuss in my paper, based on the latest research that you are aware of.
> 请根据你所了解的最新研究，为我提供一些独特的见解以便我在论文中进行讨论。

### **5.深度分析与评估**

> Please help me to conduct an in-depth analysis of these research methods and data, and provide me with an assessment of their advantages and disadvantages. 请帮助我对这些研究方法和数据进行深度分析，并为我提供关于其优缺点的评估。

### **6.提高可读性**

> Act as an academic research expert and copywriter. Your task is to review and enhance the readability of the provided [piece of text] in a research paper. Ensure that the text is clear, concise, and free from jargon while maintaining its academic integrity.
> 作为学术研究专家和文案撰写人，提高研究论文文本的可读性，确保内容清晰易懂。

### **7.寻找数据源**

- 识别和编译与研究主题相关的可信数据源，包括学术期刊、政府数据库、行业报告等。
- 提供每个数据源的简要描述，强调其相关性和可信度，并注意访问限制。

> Act as an academic research expert. Your task is to identify and compile a list of credible data sources related to [topic]. Ensure that the sources are reputable, recent, and relevant to the research objectives.
> 作为学术研究专家，识别和编译与研究主题相关的可信数据源。

### **8.寻找并了解研究方向**

- 使用GPT作为学术研究专家，对指定主题进行广泛的文献搜索，确保来源来自知名期刊、会议或学术机构。
- 提供包括标题、作者、出版日期、摘要和全文链接的论文列表。
- 对每篇论文写一个简短的总结，突出主要发现及其相关性，并确保引用来源。

> Act as an academic Research Expert. Conduct an extensive search for research papers on the specified [topic]. Ensure the papers are from reputable journals, conferences, or academic institutions. Provide a comprehensive list of the findings, including the title of the paper, authors, publication date, abstract, and a link to access the full paper. For each paper, write a brief summary highlighting the main findings and their relevance.
> 作为学术研究专家，对指定主题进行深入搜索，提供最新且权威的研究论文摘要。

### **9.总结论文要点**

- 阅读并消化指定标题的研究论文内容，提供一个简洁明了的总结，包括主要发现、方法、结果和研究意义。
- 确保总结对一般读者友好，同时保留原文的核心见解和细节。

> Act as an academic research expert. Read and digest the content of the research paper titled [title]. Produce a concise and clear summary that encapsulates the main findings, methodology, results, and implications of the study. Ensure that the summary is written in a manner that is accessible to a general audience while retaining the core insights and nuances of the original paper.
> 作为学术研究专家，阅读并总结研究论文的核心内容，使其对非专业读者也易于理解。

### **10.提出研究问题**

- 为给定主题制定一个全面的研究问题，确保问题清晰、具体且可研究。
- 考虑潜在的变量、方法和结果，确保问题在学术讨论和社会实践中具有重要性和相关性。

> Act as an academic research expert. For the given [topic], formulate a comprehensive research question that can guide a potential study. Ensure the question is clear, specific, and researchable. It should address a gap or need in the current body of knowledge, and have significance in its respective field.
> 作为学术研究专家，为给定主题制定一个清晰、具体且可研究的研究问题。

### **11.找出合适的研究方法**

- 建议适合研究主题的定性和定量研究方法，并解释每种方法的相关性和潜在优势。
- 讨论每种方法的局限性和挑战，并提供可能的解决方案或替代方法。
- 找出合适的研究方法。

> Act as an academic research expert. Your task is to suggest appropriate methodologies for researching [topic]. Provide a comprehensive list of both qualitative and quantitative research methods that are best suited for the subject matter.
> 作为学术研究专家，建议适合研究主题的定性和定量研究方法，并解释其适用性。
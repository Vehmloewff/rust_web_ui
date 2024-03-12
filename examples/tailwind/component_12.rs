use rust_web_ui::*;

pub struct Example12;

pub struct Example12Props {}

impl Default for Example12Props {
	fn default() -> Example12Props {
		Example12Props { }
	}
}

impl Widget<'_> for Example12 {
	type Props = Example12Props;

	fn render(mut ctx: Ctx<'_>, props: Example12Props) {
		ctx.styles(&[NoStyle::Noop("")]);
		ctx.set_attribute("style", "");
		
		ctx.child("1", Dynamic).run("div", |props| {
			props.styles(&[Style::Flex, Style::JustifyCenter, Style::Color(Color::Fg(11)), Style::Padding(32)]);
			props.set_attribute("style", "min-height: 600px");
			
			props.child("1", Dynamic).run("div", |props| {
				props.styles(&[NoStyle::Noop("mx-auto"), NoStyle::Noop("w-full"), NoStyle::Noop("max-w-xs")]);
				
				props.child("1", Dynamic).run("div", |props| {
					props.set_attribute("x-data", r#"Components.listbox({ modelName: 'selected', open: true, selectedIndex: 3, activeIndex: 3, items: [{"id":1,"name":"Wade Cooper","avatar":"https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":2,"name":"Arlene Mccoy","avatar":"https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":3,"name":"Devon Webb","avatar":"https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.25&w=256&h=256&q=80"},{"id":4,"name":"Tom Cook","avatar":"https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":5,"name":"Tanya Fox","avatar":"https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":6,"name":"Hellen Schmidt","avatar":"https://images.unsplash.com/photo-1487412720507-e7ab37603c6f?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":7,"name":"Caroline Schultz","avatar":"https://images.unsplash.com/photo-1568409938619-12e139227838?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":8,"name":"Mason Heaney","avatar":"https://images.unsplash.com/photo-1531427186611-ecfd6d936c79?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":9,"name":"Claudie Smitham","avatar":"https://images.unsplash.com/photo-1584486520270-19eca1efcce5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"},{"id":10,"name":"Emil Schaefer","avatar":"https://images.unsplash.com/photo-1561505457-3bcad021f8ee?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"}] })"#);
					props.set_attribute("x-init", "init()");
					
					props.child("1", Dynamic).run("label", |props| {
						props.set_attribute("id", "listbox-label");
						props.styles(&[Style::Block, NoStyle::Noop("text-sm"), NoStyle::Noop("font-medium"), NoStyle::Noop("leading-6"), Style::TextColor(Color::Fg(100))]);
						props.set_attribute("@click", "$refs.button.focus()");
						
						props.child("0", Label).run(|props| props.set_text("Assigned to"));
					});
					props.child("3", Dynamic).run("div", |props| {
						props.styles(&[NoStyle::Noop("relative"), Style::MarginTop(8)]);
						
						props.child("1", Dynamic).run("button", |props| {
							props.set_attribute("type", "button");
							props.styles(&[NoStyle::Noop("relative"), NoStyle::Noop("w-full"), NoStyle::Noop("cursor-default"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingY(6), Style::PaddingLeft(12), Style::PaddingRight(40), NoStyle::Noop("text-left"), Style::TextColor(Color::Fg(100)), NoStyle::Noop("shadow-sm"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-inset"), NoStyle::Noop("ring-gray-300"), Action::Hover(&[NoStyle::Noop("outline-none")]), Action::Hover(&[NoStyle::Noop("ring-2")]), Action::Hover(&[NoStyle::Noop("ring-indigo-500")]), Screen::Small(&[NoStyle::Noop("text-sm")]), Screen::Small(&[NoStyle::Noop("leading-6")])]);
							props.set_attribute("x-ref", "button");
							props.set_attribute("@keydown.arrow-up.stop.prevent", "onButtonClick()");
							props.set_attribute("@keydown.arrow-down.stop.prevent", "onButtonClick()");
							props.set_attribute("@click", "onButtonClick()");
							props.set_attribute("aria-haspopup", "listbox");
							props.set_attribute(":aria-expanded", "open");
							props.set_attribute("aria-expanded", "true");
							props.set_attribute("aria-labelledby", "listbox-label");
							
							props.child("1", Dynamic).run("span", |props| {
								props.styles(&[Style::Flex, Style::ItemsCenter]);
								
								props.child("1", Dynamic).run("img", |props| {
									props.set_attribute(":src", "selected.avatar");
									props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
									props.set_attribute("alt", "");
									props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-text", "selected.name");
									props.styles(&[Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
									
									props.child("0", Label).run(|props| props.set_text("Tom Cook"));
								});
							});
							props.child("3", Dynamic).run("span", |props| {
								props.styles(&[NoStyle::Noop("pointer-events-none"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::MarginLeft(12), Style::Flex, Style::ItemsCenter, Style::PaddingRight(8)]);
								
								props.child("1", Icon).run(|props| {
									props.style(&[Style::Width(20), Style::Width(20), Style::TextColor(Color::Fg(44))]);
								});
							});
						});
						props.child("3", Dynamic).run("ul", |props| {
							props.set_attribute("x-show", "open");
							props.set_attribute("x-transition:leave", "transition ease-in duration-100");
							props.set_attribute("x-transition:leave-start", "opacity-100");
							props.set_attribute("x-transition:leave-end", "opacity-0");
							props.styles(&[NoStyle::Noop("absolute"), NoStyle::Noop("z-10"), Style::MarginTop(4), NoStyle::Noop("max-h-56"), NoStyle::Noop("w-full"), NoStyle::Noop("overflow-auto"), NoStyle::Noop("rounded-md"), NoStyle::Noop("bg-white"), Style::PaddingY(4), NoStyle::Noop("text-base"), NoStyle::Noop("shadow-lg"), NoStyle::Noop("ring-1"), NoStyle::Noop("ring-black"), NoStyle::Noop("ring-opacity-5"), Action::Hover(&[NoStyle::Noop("outline-none")]), Screen::Small(&[NoStyle::Noop("text-sm")])]);
							props.set_attribute("x-max", "1");
							props.set_attribute("@click.away", "open = false");
							props.set_attribute("x-description", "Select popover, show/hide based on select state.");
							props.set_attribute("@keydown.enter.stop.prevent", "onOptionSelect()");
							props.set_attribute("@keydown.space.stop.prevent", "onOptionSelect()");
							props.set_attribute("@keydown.escape", "onEscape()");
							props.set_attribute("@keydown.arrow-up.prevent", "onArrowUp()");
							props.set_attribute("@keydown.arrow-down.prevent", "onArrowDown()");
							props.set_attribute("x-ref", "listbox");
							props.set_attribute("tabindex", "-1");
							props.set_attribute("role", "listbox");
							props.set_attribute("aria-labelledby", "listbox-label");
							props.set_attribute(":aria-activedescendant", "activeDescendant");
							props.set_attribute("aria-activedescendant", "listbox-option-3");
							
							props.child("1", Dynamic).run("li", |props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), NoStyle::Noop("relative"), NoStyle::Noop("cursor-default"), NoStyle::Noop("select-none"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(36)]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-0");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(0)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 0)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 0, 'text-gray-900': !(activeIndex === 0) }");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1491528323818-fdd1faba62cc?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[NoStyle::Noop("font-normal"), Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 0, 'font-normal': !(selectedIndex === 0) }");
										
										props.child("0", Label).run(|props| props.set_text("Wade Cooper"));
									});
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[NoStyle::Noop("text-indigo-600"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(16)]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 0, 'text-indigo-600': !(activeIndex === 0) }");
									props.set_attribute("x-show", "selectedIndex === 0");
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20)]);
									});
								});
							});
							props.child("3", Dynamic).run("li", |props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), NoStyle::Noop("relative"), NoStyle::Noop("cursor-default"), NoStyle::Noop("select-none"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(36)]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-1");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(1)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 1)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 1, 'text-gray-900': !(activeIndex === 1) }");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1550525811-e5869dd03032?ixlib=rb-1.2.1&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[NoStyle::Noop("font-normal"), Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 1, 'font-normal': !(selectedIndex === 1) }");
										
										props.child("0", Label).run(|props| props.set_text("Arlene Mccoy"));
									});
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[NoStyle::Noop("text-indigo-600"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(16)]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 1, 'text-indigo-600': !(activeIndex === 1) }");
									props.set_attribute("x-show", "selectedIndex === 1");
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20)]);
									});
								});
							});
							props.child("5", Dynamic).run("li", |props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), NoStyle::Noop("relative"), NoStyle::Noop("cursor-default"), NoStyle::Noop("select-none"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(36)]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-2");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(2)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 2)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 2, 'text-gray-900': !(activeIndex === 2) }");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1500648767791-00dcc994a43e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2.25&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[NoStyle::Noop("font-normal"), Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 2, 'font-normal': !(selectedIndex === 2) }");
										
										props.child("0", Label).run(|props| props.set_text("Devon Webb"));
									});
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[NoStyle::Noop("text-indigo-600"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(16)]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 2, 'text-indigo-600': !(activeIndex === 2) }");
									props.set_attribute("x-show", "selectedIndex === 2");
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20)]);
									});
								});
							});
							props.child("7", Dynamic).run("li", |props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), NoStyle::Noop("relative"), NoStyle::Noop("cursor-default"), NoStyle::Noop("select-none"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(36)]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-3");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(3)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 3)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 3, 'text-gray-900': !(activeIndex === 3) }");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[NoStyle::Noop("font-normal"), Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 3, 'font-normal': !(selectedIndex === 3) }");
										
										props.child("0", Label).run(|props| props.set_text("Tom Cook"));
									});
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[NoStyle::Noop("text-indigo-600"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(16)]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 3, 'text-indigo-600': !(activeIndex === 3) }");
									props.set_attribute("x-show", "selectedIndex === 3");
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20)]);
									});
								});
							});
							props.child("9", Dynamic).run("li", |props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), NoStyle::Noop("relative"), NoStyle::Noop("cursor-default"), NoStyle::Noop("select-none"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(36)]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-4");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(4)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 4)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 4, 'text-gray-900': !(activeIndex === 4) }");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[NoStyle::Noop("font-normal"), Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 4, 'font-normal': !(selectedIndex === 4) }");
										
										props.child("0", Label).run(|props| props.set_text("Tanya Fox"));
									});
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[NoStyle::Noop("text-indigo-600"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(16)]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 4, 'text-indigo-600': !(activeIndex === 4) }");
									props.set_attribute("x-show", "selectedIndex === 4");
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20)]);
									});
								});
							});
							props.child("11", Dynamic).run("li", |props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), NoStyle::Noop("relative"), NoStyle::Noop("cursor-default"), NoStyle::Noop("select-none"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(36)]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-5");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(5)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 5)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 5, 'text-gray-900': !(activeIndex === 5) }");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1487412720507-e7ab37603c6f?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[NoStyle::Noop("font-normal"), Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 5, 'font-normal': !(selectedIndex === 5) }");
										
										props.child("0", Label).run(|props| props.set_text("Hellen Schmidt"));
									});
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[NoStyle::Noop("text-indigo-600"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(16)]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 5, 'text-indigo-600': !(activeIndex === 5) }");
									props.set_attribute("x-show", "selectedIndex === 5");
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20)]);
									});
								});
							});
							props.child("13", Dynamic).run("li", |props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), NoStyle::Noop("relative"), NoStyle::Noop("cursor-default"), NoStyle::Noop("select-none"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(36)]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-6");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(6)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 6)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 6, 'text-gray-900': !(activeIndex === 6) }");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1568409938619-12e139227838?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[NoStyle::Noop("font-normal"), Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 6, 'font-normal': !(selectedIndex === 6) }");
										
										props.child("0", Label).run(|props| props.set_text("Caroline Schultz"));
									});
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[NoStyle::Noop("text-indigo-600"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(16)]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 6, 'text-indigo-600': !(activeIndex === 6) }");
									props.set_attribute("x-show", "selectedIndex === 6");
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20)]);
									});
								});
							});
							props.child("15", Dynamic).run("li", |props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), NoStyle::Noop("relative"), NoStyle::Noop("cursor-default"), NoStyle::Noop("select-none"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(36)]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-7");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(7)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 7)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 7, 'text-gray-900': !(activeIndex === 7) }");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1531427186611-ecfd6d936c79?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[NoStyle::Noop("font-normal"), Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 7, 'font-normal': !(selectedIndex === 7) }");
										
										props.child("0", Label).run(|props| props.set_text("Mason Heaney"));
									});
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[NoStyle::Noop("text-indigo-600"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(16)]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 7, 'text-indigo-600': !(activeIndex === 7) }");
									props.set_attribute("x-show", "selectedIndex === 7");
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20)]);
									});
								});
							});
							props.child("17", Dynamic).run("li", |props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), NoStyle::Noop("relative"), NoStyle::Noop("cursor-default"), NoStyle::Noop("select-none"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(36)]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-8");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(8)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 8)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 8, 'text-gray-900': !(activeIndex === 8) }");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1584486520270-19eca1efcce5?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[NoStyle::Noop("font-normal"), Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 8, 'font-normal': !(selectedIndex === 8) }");
										
										props.child("0", Label).run(|props| props.set_text("Claudie Smitham"));
									});
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[NoStyle::Noop("text-indigo-600"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(16)]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 8, 'text-indigo-600': !(activeIndex === 8) }");
									props.set_attribute("x-show", "selectedIndex === 8");
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20)]);
									});
								});
							});
							props.child("19", Dynamic).run("li", |props| {
								props.set_attribute("x-state:on", "Highlighted");
								props.set_attribute("x-state:off", "Not Highlighted");
								props.styles(&[Style::TextColor(Color::Fg(100)), NoStyle::Noop("relative"), NoStyle::Noop("cursor-default"), NoStyle::Noop("select-none"), Style::PaddingY(8), Style::PaddingLeft(12), Style::PaddingRight(36)]);
								props.set_attribute("x-description", "Select option, manage highlight styles based on mouseenter/mouseleave and keyboard navigation.");
								props.set_attribute("id", "listbox-option-9");
								props.set_attribute("role", "option");
								props.set_attribute("@click", "choose(9)");
								props.set_attribute("@mouseenter", "onMouseEnter($event)");
								props.set_attribute("@mousemove", "onMouseMove($event, 9)");
								props.set_attribute("@mouseleave", "onMouseLeave($event)");
								props.set_attribute(":class", "{ 'bg-indigo-600 text-white': activeIndex === 9, 'text-gray-900': !(activeIndex === 9) }");
								
								props.child("1", Dynamic).run("div", |props| {
									props.styles(&[Style::Flex, Style::ItemsCenter]);
									
									props.child("1", Dynamic).run("img", |props| {
										props.set_attribute("src", "https://images.unsplash.com/photo-1561505457-3bcad021f8ee?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80");
										props.set_attribute("alt", "");
										props.styles(&[Style::Width(20), Style::Width(20), NoStyle::Noop("flex-shrink-0"), NoStyle::Noop("rounded-full")]);
									});
									props.child("3", Dynamic).run("span", |props| {
										props.set_attribute("x-state:on", "Selected");
										props.set_attribute("x-state:off", "Not Selected");
										props.styles(&[NoStyle::Noop("font-normal"), Style::MarginLeft(12), Style::Block, NoStyle::Noop("truncate")]);
										props.set_attribute(":class", "{ 'font-semibold': selectedIndex === 9, 'font-normal': !(selectedIndex === 9) }");
										
										props.child("0", Label).run(|props| props.set_text("Emil Schaefer"));
									});
								});
								props.child("3", Dynamic).run("span", |props| {
									props.set_attribute("x-description", "Checkmark, only display for selected option.");
									props.set_attribute("x-state:on", "Highlighted");
									props.set_attribute("x-state:off", "Not Highlighted");
									props.styles(&[NoStyle::Noop("text-indigo-600"), NoStyle::Noop("absolute"), NoStyle::Noop("inset-y-0"), NoStyle::Noop("right-0"), Style::Flex, Style::ItemsCenter, Style::PaddingRight(16)]);
									props.set_attribute(":class", "{ 'text-white': activeIndex === 9, 'text-indigo-600': !(activeIndex === 9) }");
									props.set_attribute("x-show", "selectedIndex === 9");
									
									props.child("1", Icon).run(|props| {
										props.style(&[Style::Width(20), Style::Width(20)]);
									});
								});
							});
						});
					});
				});
			});
		});
	}
}
